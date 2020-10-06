use crate::game::game_state::*;
use crate::util::*;

/// Play a round of the game as a victim, passing in the current game state.
pub fn play_victim(state : &mut GameState) -> (usize, usize)
{
    // Determine round type
    match state.round_type
    {
        // A normal round
        RoundType::Normal =>
        {
            // Flavor message
            println!("You carefully navigate the grounds of Camp Misty, searching for car parts...");
            println!("Which location would you like to go to?");

            // Print all sections and construct vec with all section characters
            let mut section_chars = Vec::<char>::new();
            for section in &state.sections
            {
                println!("The {}?", section.name);
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
                println!("The {}?", sub_section.name);
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

        // Chase round!
        RoundType::Chase(section) =>
        {
            // Flavor message
            println!("Oh no! The killer is right on your tail!");
            println!("Which spot would you like hide in?");

            // Print all sub sections and construct a vec with all sub section characters
            let mut sub_section_chars = Vec::<char>::new();
            for sub_section in &state.sections[section].sub_sections
            {
                println!("The {}?", sub_section.name);
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
    }
}