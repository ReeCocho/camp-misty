use super::packets::*;
use crate::game::game_state::*;
use crate::game::killer_user::*;
use crate::game::victim_user::*;
use crate::util::*;

/// Play the game with another user over the internet.
///
/// Takes in our 'player_type' and a 'stream' to send our moves over, along with
/// the current game 'state'.
///
/// Returns 'true' if the game is finished and 'false' otherwise.
pub fn net_play(
    player_type: PlayerType,
    state: &mut GameState,
    stream: &mut std::net::TcpStream,
) -> bool {
    // Play game
    let our_move = if player_type == PlayerType::Killer {
        play_killer(state)
    } else {
        play_victim(state)
    };

    // Send our move to the other player
    write_over_tcp::<MovePacket>(
        stream,
        &MovePacket {
            0: our_move.0 as u32,
            1: our_move.1 as u32,
        },
    );

    // Wait for other player to tell us their move
    println!("Waiting for the other player move...");
    let other_player_move = read_over_tcp::<MovePacket>(stream);

    // Submit moves to the game state
    let res = if player_type == PlayerType::Killer {
        state.play(
            (other_player_move.0 as usize, other_player_move.1 as usize),
            our_move,
        )
    } else {
        state.play(
            our_move,
            (other_player_move.0 as usize, other_player_move.1 as usize),
        )
    };

    // Place trap if evaded
    if res.result == RoundResult::Evaded {
        // If we are the victim, place teh trap and then tell the other player where the trap was placed
        if player_type == PlayerType::Victim {
            let trap_loc = victim_place_trap(state) as TrapPacket;
            write_over_tcp::<TrapPacket>(stream, &trap_loc);
        }
        // If we are the killer, have the other player tell us where they placed the trap
        else {
            let trap_loc = read_over_tcp::<TrapPacket>(stream);
            state.place_trap(trap_loc as usize);
        }
    }
    // Killer wins
    else if res.result == RoundResult::Caught {
        killer_win_message(player_type);
        return true;
    }
    // Victim wins
    else if res.result == RoundResult::AllPartsFound {
        victim_win_message(player_type);
        return true;
    }

    // Nobody won
    false
}
