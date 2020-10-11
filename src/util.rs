use crate::multiplayer::packets::*;

/// Helper function to read user input.
pub fn read_str() -> String {
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Flush failed!");

    // Get port as string
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input.");

    String::from(input.trim())
}

/// Helper function to have the user pick a character from a list of valid choices.
///
/// The first argument is an array of valid uppercase characters to choose from.
///
/// The second argument is the message to prompt the user with when an invalid character is chosen.
///
/// The function returns the chosen character.
pub fn pick_char(valid_chars: &[char], err_msg: &str) -> char {
    // Loop to constantly as for input
    loop {
        // Read input
        let input = read_str();

        // Must be a single character
        if input.len() == 1 {
            // Get the uppercase version of the letter
            // Guaranteed not to panic since we have at least 1 character
            let upper = input.chars().next().unwrap().to_uppercase().next().unwrap();

            // Loop over all valid chars
            for c in valid_chars {
                // If a valid char was entered, return it
                if upper == *c {
                    return *c;
                }
            }
        }

        // Unable to read input
        println!("{}", err_msg);
    }
}

/// Prints a message when the vitim wins.
///
/// The only arguments is the type of player "we" are.
pub fn victim_win_message(player_type: PlayerType) {
    if player_type == PlayerType::Victim {
        println!("Yes!!! You found all of the car parts and are able to escape Camp Misty!");
        print_win();
    } else {
        println!("No!!! The victim found all the car parts and escaped Camp Misty!");
        print_lose();
    }
}

/// Prints a message when the killer wins.
///
/// The only arguments is the type of player "we" are.
pub fn killer_win_message(player_type: PlayerType) {
    if player_type == PlayerType::Victim {
        println!("Noooo!!! The killer slices your back and you fall dead...");
        print_lose();
    } else {
        println!("Muahahahaha!!! You slice the victim across their back, and they fall dead...");
        print_win();
    }
}

/// Print the title screen
pub fn print_title_screen() {
    println!(
        " 
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
                                    ░ ░ "
    );
}

// Print win message
pub fn print_win() {
    println!(
        "\n
▀▄    ▄ ████▄   ▄        ▄ ▄   ▄█    ▄  
  █  █  █   █    █      █   █  ██     █ 
   ▀█   █   █ █   █    █ ▄   █ ██ ██   █
   █    ▀████ █   █    █  █  █ ▐█ █ █  █
 ▄▀           █▄ ▄█     █ █ █   ▐ █  █ █
               ▀▀▀       ▀ ▀      █   ██"
    );
}

// Print lose message
pub fn print_lose() {
    println!(
        "\n
▓██   ██▓ ▒█████   █    ██     ██▓     ▒█████    ██████ ▓█████ 
 ▒██  ██▒▒██▒  ██▒ ██  ▓██▒   ▓██▒    ▒██▒  ██▒▒██    ▒ ▓█   ▀ 
  ▒██ ██░▒██░  ██▒▓██  ▒██░   ▒██░    ▒██░  ██▒░ ▓██▄   ▒███   
  ░ ▐██▓░▒██   ██░▓▓█  ░██░   ▒██░    ▒██   ██░  ▒   ██▒▒▓█  ▄ 
  ░ ██▒▓░░ ████▓▒░▒▒█████▓    ░██████▒░ ████▓▒░▒██████▒▒░▒████▒
   ██▒▒▒ ░ ▒░▒░▒░ ░▒▓▒ ▒ ▒    ░ ▒░▓  ░░ ▒░▒░▒░ ▒ ▒▓▒ ▒ ░░░ ▒░ ░
 ▓██ ░▒░   ░ ▒ ▒░ ░░▒░ ░ ░    ░ ░ ▒  ░  ░ ▒ ▒░ ░ ░▒  ░ ░ ░ ░  ░
 ▒ ▒ ░░  ░ ░ ░ ▒   ░░░ ░ ░      ░ ░   ░ ░ ░ ▒  ░  ░  ░     ░   
 ░ ░         ░ ░     ░            ░  ░    ░ ░        ░     ░  ░
 ░ ░                                                           "
    )
}
