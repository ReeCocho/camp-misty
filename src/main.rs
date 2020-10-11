pub mod game;
pub mod multiplayer;
pub mod util;

use multiplayer::client::*;
use multiplayer::server::*;

fn main() {
    // Title screen
    util::print_title_screen();

    // Game loop over choices
    loop {
        // Ask for host, client, instructions, or quit
        println!("          (H)ost a game");
        println!("          (J)oin a game");
        println!("          (I)nstructions");
        println!("          (Q)uit");

        // Determine selection
        match util::pick_char(&['H', 'J', 'Q', 'I'], "Sorry, that isn't an option.") {
            // Host a game
            'H' => {
                // Host game
                Server::host_game();

                // Print title screen for main menu when finished
                util::print_title_screen();
            }

            // Join a game
            'J' => {
                // Join game
                Client::join_game();

                // Print title screen for main menu when finished
                util::print_title_screen();
            }

            // Quit
            'Q' => {
                // End the game
                println!("Thanks for playing!");
                return;
            }

            // Instructions
            'I' => {
                println!("Trapped within this hellish domain is a victim, who is being hunted down by a ruthless killer!");
                println!("The victim is trying to find 5 car parts so that they can repair their vehicle and escape.");
                println!("The killer is trying to stop the victim.\n");

                println!("Layout of Camp Misty:");
                println!("Camp Misty is broken up into five different locations: The Cabin, the Abandoned Manor, the Bonfire, the Old Forest, and Lake Misty itself.");
                println!("Within each of these locations are five different spots. Each spot may or may not contain a car part that the victim needs.\n");

                println!("Goal of the Victim:");
                println!("You are trying to find 5 car parts.");
                println!("There is exactly one car part in each location, so if you find one in a location, you don’t need to keep checking it.");
                println!("Try to choose locations and spots as randomly as you can so that the killer can’t predict where you will go next!\n");

                println!("Goal of the Killer:");
                println!("You are trying to hunt down the victim.");
                println!("Each time the victim finds a car part, you will be alerted as to which location the part was found in.");
                println!("That means you don’t have to check that location anymore because the victim won’t be searching for parts there.\n");

                println!("The Chase:");
                println!("In the event that the victim and killer choose the same location, but not the same spot, a chase will begin.");
                println!("On the next round, the victim will need to hide in a spot in the same location, and the killer will search for the victim.\n");

                println!("Traps:");
                println!("If the victim is able to avoid the killer during a chase, they will get a trap.");
                println!("This trap is deployed in one of the five locations.");
                println!("If the killer chooses to search the trapped location, they will be unable to capture the victim or initiate a chase, even if they choose the same exact spot.\n");

                println!("Winning:");
                println!("If the killer choses the same location and spot as the victim, they will wound the victim.");
                println!("If the victim is wounded two times, they die and the killer wins.");
                println!("If the victim is able to find all five car parts, they win the game.\n");

                println!("Enter anything to return to the main menu...");
                util::read_str();

                // Print title screen for main menu when finished
                util::print_title_screen();
            }

            // Unknown
            _ => panic!("Invalid input"),
        }
    }
}
