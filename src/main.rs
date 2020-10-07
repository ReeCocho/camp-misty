pub mod game;
pub mod packets;
pub mod server;
pub mod util;

use game::game_state::*;
use game::section::*;
use game::sub_section::*;
use game::killer_ai::*;
use game::victim_ai::*;
use server::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

fn main() 
{
    // Title screen
    print_title_screen();

    // Game loop over choices
    loop
    {
        // Ask for host, client, or quit
        println!("          (H)ost a game");
        println!("          (J)oin a a game");
        println!("          (Q)uit");

        // Determine selection
        match util::pick_char(&vec!['H', 'J', 'Q'], "Sorry, that isn't an option.")
        {
            // Host a game
            'H' =>
            {
                // Total number of wins of each player type
                let mut killer_wins : usize = 0;
                let mut victim_wins : usize = 0;

                // Play 1,000 matches
                for i in 0..10000
                {
                    // Create a game state
                    let mut state = Rc::new(RefCell::new(GameState::new()));

                    // Create AI plays
                    let mut killer = KillerAI::new(state.clone());
                    let mut victim = VictimAI::new(state.clone());

                    // Play game until there is a winner
                    loop
                    {
                        // Have each AI make a move
                        let killer_move = killer.play();
                        let victim_move = victim.play();

                        // Submit moves to the game state
                        let res = state.borrow_mut().play(victim_move, killer_move).expect("Something went wrong during play");

                        // Break if someone won
                        if res.0 == game::game_state::RoundResult::Caught
                        {
                            killer_wins += 1;
                            break;
                        }
                        else if res.0 == game::game_state::RoundResult::AllPartsFound
                        {
                            victim_wins += 1;
                            break;
                        }
                    }
                }

                // Compute win ratio
                println!("V/K Win Ratio: {}", (victim_wins as f32) / (killer_wins as f32));

                // // Host game
                // host_game();

                // // Print title screen for main menu
                // print_title_screen();
            }

            // Join a game
            'J' =>
            {
                // Print title screen for main menu
                print_title_screen();
            }

            // Quit
            'Q' =>
            {
                // End the game
                println!("Thanks for playing!");
                return;
            }

            // Unknown
            _ => panic!("Invalid input")
        }
    }

    // Create game state
    let mut state = GameState::new();

    // Take a list of all car parts and shuffle them
    let car_parts = [
        CarPart::Battery, 
        CarPart::Gasoline, 
        CarPart::Headlights, 
        CarPart::SparkPlug];
    
    // Must be one less part than the number of sections
    assert_eq!(car_parts.len(), SECTION_COUNT - 1);

    // Create a list of indices
    let mut part_loc_inds : Vec<usize> = (0..SECTION_COUNT).collect();

    // Remove a random index from the list (this indicates which section doesn't have a part)
    part_loc_inds.remove(rand::thread_rng().gen_range(0, part_loc_inds.len()));

    // Shuffle the index. Then, we can distribute the parts randomly among the sections
    part_loc_inds.shuffle(&mut rand::thread_rng());

    // Distribute car parts
    let mut part_index : usize = 0;
    for i in part_loc_inds
    {
        // Randomly choose which sub section gets the part
        let rand_ind = rand::thread_rng().gen_range(0, SUB_SECTION_COUNT);

        // Place the part in the sub section
        state.hide_part(i, rand_ind, car_parts[part_index]);

        // Increment part index
        part_index += 1;
    }

    // Print out sub sections with hidden parts
    for section in state.sections.iter()
    {
        for sub_section in section.sub_sections.iter()
        {
            if sub_section.part != CarPart::None
            {
                println!("{} {}", section.name, sub_section.name);
            }
        }
    }
}



/// Print the title screen
fn print_title_screen()
{
    println!(" 
            Welcome To...\n\n\
    ▄████▄  ▄▄▄      ███▄ ▄███▓██▓███    \n\
    ▒██▀ ▀█ ▒████▄   ▓██▒▀█▀ ██▓██░  ██▒ \n\
    ▒▓█    ▄▒██  ▀█▄ ▓██    ▓██▓██░ ██▓▒ \n\
    ▒▓▓▄ ▄██░██▄▄▄▄██▒██    ▒██▒██▄█▓▒ ▒ \n\
    ▒ ▓███▀ ░▓█   ▓██▒██▒   ░██▒██▒ ░  ░ \n\
    ░ ░▒ ▒  ░▒▒   ▓▒█░ ▒░   ░  ▒▓▒░ ░  ░ \n\
        ░  ▒    ▒   ▒▒ ░  ░      ░▒ ░    \n\
    ░         ░   ▒  ░      ░  ░░        \n\
    ░███▄ ▄███▓██▓░██████▄▄▄█████▓██   ██▓ \n\
    ▓██▒▀█▀ ██▓██▒██    ▒▓  ██▒ ▓▒▒██  ██▒ \n\
    ▓██    ▓██▒██░ ▓██▄  ▒ ▓██░ ▒░ ▒██ ██░ \n\
    ▒██    ▒██░██░ ▒   ██░ ▓██▓ ░  ░ ▐██▓░ \n\
    ▒██▒   ░██░██▒██████▒▒ ▒██▒ ░  ░ ██▒▓░ \n\
    ░ ▒░   ░  ░▓ ▒ ▒▓▒ ▒ ░ ▒ ░░     ██▒▒▒  \n\
    ░  ░      ░▒ ░ ░▒  ░ ░   ░    ▓██ ░▒░  \n\
    ░      ░   ▒ ░  ░  ░   ░      ▒ ▒ ░░   \n\
            ░   ░       ░          ░ ░     \n\
                                    ░ ░ ");
}

/// Host game logic
fn host_game()
{
    // Loop to make server
    let mut server : Server;
    loop
    {
        // Ask for port number
        println!("Please enter the port you wish to use.");

        // Loop to get port
        let mut port : u32 = 0;
        loop
        {
            // Convert to int
            match util::read_str().parse::<u32>()
            {
                // Port is valid
                Ok(i) =>
                {
                    port = i;
                    break;
                }

                // There was a problem
                Err(e) =>
                {
                    println!("Sorry, I didn't understand you.");
                }
            }
        }

        // Attempt to create server
        match Server::new(port)
        {
            // Created successfully
            Ok(s) =>
            {
                server = s;
                break;
            }

            // There was a problem
            Err(e) =>
            {
                println!("There was a problem creating the server.");
                println!("Would you like to (T)ry again or (R)eturn to the main menu?");

                match util::pick_char(&vec!['T', 'R'], "Sorry, that isn't an option.")
                {
                    'T' => {}
                    'R' => { return; }
                    _ => panic!("Invalid input.")
                }
            }
        }
    }

    // Wait for client
    println!("Waiting for client...");
    match server.wait_for_client()
    {
        // Client joined successfully
        Ok(_) =>  {}
        Err(_) => 
        {
            println!("Woops! Looks like something went wrong when the client tried to connect. Returning to the main menu.");
            return;
        }
    }
}

/// Join game logic
fn join_game()
{

}