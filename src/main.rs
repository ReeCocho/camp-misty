pub mod game;
pub mod multiplayer;
pub mod util;

use multiplayer::server::*;
use multiplayer::client::*;

fn main() 
{
    // Title screen
    print_title_screen();

    // Game loop over choices
    loop
    {
        // Ask for host, client, or quit
        println!("          (H)ost a game");
        println!("          (J)oin a game");
        println!("          (Q)uit");

        // Determine selection
        match util::pick_char(&vec!['H', 'J', 'Q'], "Sorry, that isn't an option.")
        {
            // Host a game
            'H' =>
            {
                // Host game
                Server::host_game();

                // Print title screen for main menu
                print_title_screen();
            }

            // Join a game
            'J' =>
            {
                // Join game
                Client::join_game();

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