use rand::Rng;

use crate::game::game_state::*;
use crate::game::section::*;

/// An AI version of the killer to be used in testing/single player
pub struct KillerAI {
    /// List of sections to check
    sections: Vec<usize>,
}

impl Default for KillerAI {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> KillerAI {
    /// Constructor.
    ///
    /// Only argument is the game state.
    pub fn new() -> KillerAI {
        KillerAI {
            sections: (0..SECTION_COUNT).collect(),
        }
    }

    /// Play a round of the game as the killer.
    pub fn play(&mut self, state: &GameState) -> (usize, usize) {
        // Get last game result values
        let last_result = &state.last_result;

        // If a car part was found in the last round, remove that section
        // from our list of sections to check
        if last_result.1 != SECTION_COUNT {
            if let Some(i) = self.sections.iter().position(|&s| s == last_result.1) {
                self.sections.remove(i);
            }
        }

        // Determine move based off of last round result
        match last_result.0 {
            // Normal round logic
            RoundResult::Nothing | RoundResult::Evaded | RoundResult::Wounded => {
                // Choose a random section from our list of valid sections
                let sec_ind = rand::thread_rng().gen_range(0, self.sections.len());
                let section = self.sections[sec_ind];

                // Choose a random sub section within the section
                let sub_section = rand::thread_rng().gen_range(0, SUB_SECTION_COUNT);

                // Play that move
                (section, sub_section)
            }

            // Special logic for a chase
            RoundResult::ChaseBegins(section) => {
                // Choose a random sub section within the section
                let sub_sec_ind = rand::thread_rng().gen_range(0, SUB_SECTION_COUNT);

                // Play that move in the chase section
                (section, sub_sec_ind)
            }

            // All other scenarios result in a default move
            _ => (0, 0),
        }
    }
}
