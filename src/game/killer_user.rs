use crate::game::game_state::*;
use crate::util::*;

/// Play a round of the game as a killer by passing in the current game state.
pub fn play_killer(state : &mut GameState) -> (usize, usize)
{
    // Convenience function for special print out
    let found_part_msg = 
    || if state.last_result.1 != SECTION_COUNT 
    { 
        println!(
            "Oh no! The victim found a car part in the {}!", 
            state.sections[state.last_result.1].name); 
    };

    // Print out a special message depending on what happened last round
    match state.last_result.0
    {
        RoundResult::AllPartsFound => 
        {
            println!("Oh no! The victim found all the car parts and was able to escape! You lose!");
        }

        RoundResult::Caught => 
        {
            println!("Yes! You caught the victim, sinking your blade deep into their back... You win!");
        }

        RoundResult::ChaseBegins(section) => 
        {
            found_part_msg();
            println!(
                "Muahaha! You have the victim in your sights! Where in the {} would you like to search for them?", 
                state.sections[section].name);
        }

        RoundResult::Evaded => 
        {
            found_part_msg();
            println!("No, no, no! The victim got away!");
            println!("Now, which location would you like to check?");
        }

        RoundResult::Nothing => 
        {
            found_part_msg();
            println!("Paitently, you stalk the grounds of Camp Misty for your victim...");
            println!("Now, which location would you like to check?");
        }

        RoundResult::TrapTriggered => 
        {
            found_part_msg();
            println!("Oh no! You stepped right into the victims trap! You spent the round getting yourself out.");
            println!("Now, which location would you like to check?");
        }

        RoundResult::Wounded => 
        {
            found_part_msg();
            println!("Muahaha! You found the victim and were able to get a good swing in.");
            println!("They are wounded. If you find them again, you win...");
            println!("Now, which location would you like to check?");
        }
    }

    // Determine the round type
    match state.last_result.0
    {
        // Chase round!
        RoundResult::ChaseBegins(section) =>
        {
            // Print all sub sections and construct a vec with all sub section characters
            let mut sub_section_chars = Vec::<char>::new();
            for sub_section in &state.sections[section].sub_sections
            {
                println!("{}?", sub_section.name);
                sub_section_chars.push(sub_section.letter);
            }

            // Ask user for character
            let sub_section_char = pick_char(&sub_section_chars, "Sorry, that isn't a spot! Choose a spot.");

            // Get the sub section by character
            // NOTE: Again, no None checks are needed
            let sub_section_ind = state.get_sub_section_by_letter(section, sub_section_char).expect("Sub section not found!");

            // Return the section and sub section tuple
            return (section, sub_section_ind);
        }

        // Normal round
        _ =>
        {
            // Print all sections and construct vec with all section characters
            let mut section_chars = Vec::<char>::new();
            for section in &state.sections
            {
                println!("{}?", section.name);
                section_chars.push(section.letter);
            }

            // Ask user for character
            let section_char = pick_char(&section_chars, "Sorry, that isn't a location! Choose a location.");

            // Get the section by character
            // NOTE: We don't need to do a 'None' check here because the 'pick_char' function guarantees we choose
            // a valid section
            let section_ind = state.get_section_by_letter(section_char).expect("Section not found!");
            let section = &mut state.sections[section_ind];

            // Flavor message
            println!("Which spot in here would you like to check?");

            // Print all sub sections and construct a vec with all sub section characters
            let mut sub_section_chars = Vec::<char>::new();
            for sub_section in &section.sub_sections
            {
                println!("{}?", sub_section.name);
                sub_section_chars.push(sub_section.letter);
            }

            // Ask user for character
            let sub_section_char = pick_char(&sub_section_chars, "Sorry, that isn't a spot! Choose a spot.");

            // Get the sub section by character
            // NOTE: Again, no None checks are needed
            let sub_section_ind = state.get_sub_section_by_letter(section_ind, sub_section_char).expect("Sub section not found!");

            // Return the section and sub section tuple
            return (section_ind, sub_section_ind);
        }
    }
}