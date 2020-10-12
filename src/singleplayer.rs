use rand::Rng;

use crate::game::game_state::*;
use crate::game::killer_ai::*;
use crate::game::killer_user::*;
use crate::game::victim_ai::*;
use crate::game::victim_user::*;
use crate::multiplayer::packets::*;
use crate::util::*;

/// Play the game by yourself.
pub fn play_singleplayer() {
    // Choose if you want to be the killer or the victim
    println!("Would you like to be the (K)iller, the (V)ictim, or (R)andomly choose?");
    let player_type = match pick_char(&['K', 'V', 'R'], "Sorry, that isn't a valid option.") {
        'K' => PlayerType::Killer,
        'V' => PlayerType::Victim,
        'R' => {
            if rand::thread_rng().gen_range(0, 2) == 0 {
                PlayerType::Killer
            } else {
                PlayerType::Victim
            }
        }
        _ => panic!("Invalid option chosen!"),
    };

    // Create game state and generate random state
    let mut state = GameState::new();
    state.gen_state();

    // Play as either victim or killer
    match player_type {
        PlayerType::Killer => {
            // Create victim
            let mut victim = VictimAI::new(&state);

            // Play game until there is a winner
            loop {
                // Make moves
                let killer_move = play_killer(&mut state);
                let victim_move = victim.play(&mut state);

                // Submit moves to the game state
                let res = state.play(victim_move, killer_move);

                // Place trap if evaded
                if res.result == RoundResult::Evaded {
                    victim.place_trap(&mut state);
                }
                // Break if someone won
                else if res.result == RoundResult::Caught {
                    killer_win_message(player_type);
                    break;
                } else if res.result == RoundResult::AllPartsFound {
                    victim_win_message(player_type);
                    break;
                }
            }
        }

        PlayerType::Victim => {
            // Create killer
            let mut killer = KillerAI::new(&state);

            // Play game until there is a winner
            loop {
                // Make moves
                let killer_move = killer.play(&state);
                let victim_move = play_victim(&state);

                // Submit moves to the game state
                let res = state.play(victim_move, killer_move);

                // Place trap if evaded
                if res.result == RoundResult::Evaded {
                    victim_place_trap(&mut state);
                }
                // Break if someone won
                else if res.result == RoundResult::Caught {
                    killer_win_message(player_type);
                    break;
                } else if res.result == RoundResult::AllPartsFound {
                    victim_win_message(player_type);
                    break;
                }
            }
        }
    }

    // Return to title screen
    println!("Enter anything to return to the title screen...");
    read_str();
}
