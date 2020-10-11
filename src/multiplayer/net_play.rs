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
    let res: (RoundResult, usize);
    if player_type == PlayerType::Killer {
        res = state
            .play(
                (other_player_move.0 as usize, other_player_move.1 as usize),
                our_move,
            )
            .expect("Something went wrong during play");
    } else {
        res = state
            .play(
                our_move,
                (other_player_move.0 as usize, other_player_move.1 as usize),
            )
            .expect("Something went wrong during play");
    }

    // Place trap if evaded
    if res.0 == RoundResult::Evaded {
        // If we are the victim, tell the other player where the trap was placed
        if player_type == PlayerType::Victim {
            // Place the trap
            let trap_loc = victim_place_trap(state) as TrapPacket;

            // Tell the other player where we placed the trap
            write_over_tcp::<TrapPacket>(stream, &trap_loc);
        }
        // If we are the killer, have the other player tell us where they placed the trap
        else {
            let trap_loc = read_over_tcp::<TrapPacket>(stream);
            state.place_trap(trap_loc as usize);
        }
    }
    // Killer wins
    else if res.0 == RoundResult::Caught {
        if player_type == PlayerType::Victim {
            println!("Noooo!!! The killer slices your back and you fall dead...");
            print_lose();
        } else {
            println!(
                "Muahahahaha!!! You slice the victim across their back, and they fall dead..."
            );
            print_win();
        }

        return true;
    }
    // Victim wins
    else if res.0 == RoundResult::AllPartsFound {
        if player_type == PlayerType::Victim {
            println!("Yes!!! You found all of the car parts and are able to escape Camp Misty!");
            print_win();
        } else {
            println!("No!!! The victim found all the car parts and escaped Camp Misty!");
            print_lose();
        }

        return true;
    }

    // Nobody won
    false
}
