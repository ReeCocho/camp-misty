use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

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
    let state = Rc::new(RefCell::new(GameState::new()));
    state.borrow_mut().gen_state();

    // Play as either victim or killer
    match player_type {
        PlayerType::Killer => {
            // Create victim
            let mut victim = VictimAI::new(state.clone());

            // Play game until there is a winner
            loop {
                // Make moves
                let killer_move = play_killer(&mut state.borrow_mut());
                let victim_move = victim.play();

                // Submit moves to the game state
                let res = state
                    .borrow_mut()
                    .play(victim_move, killer_move)
                    .expect("Something went wrong during play");

                // Place trap if evaded
                if res.0 == RoundResult::Evaded {
                    victim.place_trap();
                }
                // Break if someone won
                else if res.0 == RoundResult::Caught {
                    killer_win_message(player_type);
                    break;
                } else if res.0 == RoundResult::AllPartsFound {
                    victim_win_message(player_type);
                    break;
                }
            }
        }

        PlayerType::Victim => {
            // Create killer
            let mut killer = KillerAI::new(state.clone());

            // Play game until there is a winner
            loop {
                // Make moves
                let killer_move = killer.play();
                let victim_move = play_victim(&state.borrow_mut());

                // Submit moves to the game state
                let res = state
                    .borrow_mut()
                    .play(victim_move, killer_move)
                    .expect("Something went wrong during play");

                // Place trap if evaded
                if res.0 == RoundResult::Evaded {
                    victim_place_trap(&mut state.borrow_mut());
                }
                // Break if someone won
                else if res.0 == RoundResult::Caught {
                    killer_win_message(player_type);
                    break;
                } else if res.0 == RoundResult::AllPartsFound {
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
