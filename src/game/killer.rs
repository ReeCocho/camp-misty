use crate::game::game_state::*;

/// Play a round of the game as a killer by passing in the current game state.
pub fn play_killer(state : &GameState)
{
    // Determine the round type
    match state.round_type
    {
        // Normal round
        RoundType::Normal =>
        {
            // Flavor message
            println!("Paitently, you stalk the grounds of Camp Misty for your victim...");
            println!("Would you like to go to...");

            // Print all sections and construct 
            for section in &state.sections
            {
                println!("The {}?", section.name);
            }
        }

        // Chase round!
        RoundType::Chase(section) =>
        {
            // Flavor message
            println!("You have the victim on the run!");
            println!("Would you like to go to...");

            // Print all sub sections in the chase section
            for sub_section in &state.sections[section].sub_sections
            {
                println!("The {}?", sub_section.name);
            }
        }
    }
}