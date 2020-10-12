use rand::Rng;

use crate::game::game_state::*;

/// An AI version of the killer to be used in testing/single player
pub struct KillerAI {
    /// List of sections to check
    sections: Vec<usize>,
}

impl<'a> KillerAI {
    /// Constructor.
    ///
    /// Only argument is the game state the killer will be playing in.
    pub fn new(state: &GameState) -> KillerAI {
        KillerAI {
            sections: (0..state.sections.len()).collect(),
        }
    }

    /// Play a round of the game as the killer.
    pub fn play(&mut self, state: &GameState) -> (usize, usize) {
        // Get last game result values
        let last_result = &state.last_result;

        // If a car part was found in the last round, remove that section
        // from our list of sections to check
        if let Some(part) = last_result.part_section_index {
            if let Some(i) = self.sections.iter().position(|&s| s == part) {
                self.sections.remove(i);
            }
        }

        // Determine move based off of last round result
        match last_result.result {
            // Normal round logic
            RoundResult::Nothing | RoundResult::Evaded | RoundResult::Wounded => {
                // Choose a random section from our list of valid sections
                let sec_ind = rand::thread_rng().gen_range(0, self.sections.len());
                let section = self.sections[sec_ind];

                // Choose a random sub section within the section
                let sub_section = rand::thread_rng().gen_range(0, state.sections.len());

                // Play that move
                (section, sub_section)
            }

            // Special logic for a chase
            RoundResult::ChaseBegins(section) => {
                // Choose a random sub section within the section
                let sub_sec_ind =
                    rand::thread_rng().gen_range(0, state.sections[section].sub_sections.len());

                // Play that move in the chase section
                (section, sub_sec_ind)
            }

            // All other scenarios result in a default move
            _ => (0, 0),
        }
    }
}
