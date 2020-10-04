pub mod game;
pub mod server;
pub mod errors;

use game::game_state::*;
use game::section::*;
use game::sub_section::*;
use rand::seq::SliceRandom;
use rand::Rng;

fn main() 
{
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