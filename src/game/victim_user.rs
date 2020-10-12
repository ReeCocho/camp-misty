use crate::game::game_state::*;
use crate::util::*;

/// Play a round of the game as a victim, passing in the current game state.
pub fn play_victim(state: &GameState) -> (usize, usize) {
    // Print out a special message depending on what happened last round
    match state.last_result.0 {
        RoundResult::ChaseBegins(section) => {
            if state.last_result.1 != SECTION_COUNT {
                println!("Nice! You found a car part!");
            }
            println!(
                "Oh no! The killer is in the {} with you! They're right behind you!",
                state.sections[section].name
            );
            println!("Where would you like to hide?");
        }

        RoundResult::Evaded => {
            if state.last_result.1 != SECTION_COUNT {
                println!("Nice! You found a car part!");
            }
            // NOTE: We don't put anything super special here because it's handled by victim_place_trap()
            println!("Now, which location would you like to check?");
        }

        RoundResult::Nothing => {
            if state.last_result.1 != SECTION_COUNT {
                println!("Nice! You found a car part!");
            }
            println!(
                "You carefully navigate the grounds of Camp Misty, searching for car parts..."
            );
            println!("Now, which location would you like to check?");
        }

        RoundResult::TrapTriggered => {
            if state.last_result.1 != SECTION_COUNT {
                println!("Nice! You found a car part!");
            }
            println!("Ha, ha, ha! You hear the killer fall into your trap!");
            println!("You were safe that round.");
            println!("Now, which location would you like to check?");
        }

        RoundResult::Wounded => {
            if state.last_result.1 != SECTION_COUNT {
                println!("Nice! You found a car part!");
            }
            println!("Oh no! You ran right into the killer and they cut you across");
            println!("the back as you tried to get away!");
            println!("You have a nasty wound. If they catch you again, you won't survive...");
            println!("Now, which location would you like to check?");
        }

        // Win conditions are ignored
        _ => {}
    }

    // Logic for chosing a location

    // Determine round type
    match state.last_result.0 {
        // Chase round!
        RoundResult::ChaseBegins(section) => {
            // Print all sub sections and construct a vec with all sub section characters
            let mut sub_section_chars = Vec::<char>::new();
            for sub_section in &state.sections[section].sub_sections {
                println!("{}?", sub_section.name);
                sub_section_chars.push(sub_section.letter);
            }

            // Ask user for character
            let sub_section_char = pick_char(
                &sub_section_chars,
                "Sorry, that isn't a spot! Choose a spot.",
            );

            // Get the sub section by character
            // NOTE: Again, no None checks are needed
            let sub_section_ind = state
                .get_sub_section_by_letter(section, sub_section_char)
                .expect("Sub section not found!");

            // Return the section and sub section tuple
            (section, sub_section_ind)
        }

        // A normal round
        _ => {
            // Print all sections and construct vec with all section characters
            let mut section_chars = Vec::<char>::new();
            for section in &state.sections {
                println!("{}?", section.name);
                section_chars.push(section.letter);
            }

            // Ask user for character
            let section_char = pick_char(
                &section_chars,
                "Sorry, that isn't a location! Choose a location.",
            );

            // Get the section by character
            // NOTE: We don't need to do a 'None' check here because the 'pick_char' function guarantees we choose
            // a valid section
            let section_ind = state
                .get_section_by_letter(section_char)
                .expect("Section not found!");
            let section = &state.sections[section_ind];

            // Flavor message
            println!("Which spot in here would you like to check?");

            // Print all sub sections and construct a vec with all sub section characters
            let mut sub_section_chars = Vec::<char>::new();
            for sub_section in &section.sub_sections {
                println!("{}?", sub_section.name);
                sub_section_chars.push(sub_section.letter);
            }

            // Ask user for character
            let sub_section_char = pick_char(
                &sub_section_chars,
                "Sorry, that isn't a spot! Choose a spot.",
            );

            // Get the sub section by character
            // NOTE: Again, no None checks are needed
            let sub_section_ind = state
                .get_sub_section_by_letter(section_ind, sub_section_char)
                .expect("Sub section not found!");

            // Return the section and sub section tuple
            (section_ind, sub_section_ind)
        }
    }
}

/// Place a trap as a victim
pub fn victim_place_trap(state: &mut GameState) -> usize {
    // Flavor
    println!("What a relief! You evaded the killer and found a trap!");
    println!("Where would you like to place it?");

    // Print all sections and construct vec with all section characters
    let mut section_chars = Vec::<char>::new();
    for section in &state.sections {
        println!("{}?", section.name);
        section_chars.push(section.letter);
    }

    // Ask user for character
    let section_char = pick_char(
        &section_chars,
        "Sorry, that isn't a location! Choose a location.",
    );

    // Get the section by character and place the trap
    // NOTE: We don't need to do a 'None' check here because the 'pick_char' function guarantees we choose
    // a valid section
    let section_ind = state
        .get_section_by_letter(section_char)
        .expect("Section not found!");

    state.place_trap(section_ind);

    // Flavor
    println!("Trapped has been placed!");

    section_ind
}
