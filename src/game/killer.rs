use crate::game::game_state::*;
use crate::util::*;

/// Play a round of the game as a killer by passing in the current game state.
pub fn play_killer(state : &mut GameState) -> KillerRoundResult
{
    // Determine the round type
    match state.last_result.0
    {
        // Chase round!
        RoundResult::ChaseBegins(section) =>
        {
            // Flavor message
            println!("You have your victim on the run!");
            println!("Which spot would you like to search for the victim at?");

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
            return KillerRoundResult::Normal((section, sub_section_ind));
        }

        // Normal round
        _ =>
        {
            // Flavor message
            println!("Paitently, you stalk the grounds of Camp Misty for your victim...");
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

            // Check if they got trapped
            if section.trapped
            {
                // Flavor message
                println!("Oh no! You were trapped! Looks like you'll have to wait this round out...");

                // Untrap the section
                section.trapped = false;

                return KillerRoundResult::Trapped;
            }

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
            return KillerRoundResult::Normal((section_ind, sub_section_ind));
        }
    }
}



/// Killer round result.
pub enum KillerRoundResult
{
    // Normal round (contains section/sub-section tuple)
    Normal((usize, usize)),

    // The killer was trapped!
    Trapped
}