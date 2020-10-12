use rand::Rng;

use crate::game::game_state::*;

/// An AI version of a victim to be used for testing/single player.
pub struct VictimAI {
    /// List of all unvisted sections and sub sections
    unvisited: Vec<(usize, usize)>,
}

impl VictimAI {
    /// Constructor.
    ///
    /// The only argument is a reference to the game state to play in.
    pub fn new(state: &GameState) -> VictimAI {
        let mut ai = VictimAI {
            unvisited: Vec::<(usize, usize)>::new(),
        };

        // Initialize unvisited tuples
        // NOTE: This means the unvisited tuples are sorted by section!
        for i in 0..state.sections.len() {
            for j in 0..state.sections[i].sub_sections.len() {
                ai.unvisited.push((i, j));
            }
        }

        ai
    }

    /// Play a round of the game.
    ///
    /// Returns a tuple containing what move the AI decided to take.
    pub fn play(&mut self, state: &mut GameState) -> (usize, usize) {
        // Determine move based off of last round result
        let tup = match state.last_result.result {
            // Normal round logic
            RoundResult::Nothing
            | RoundResult::TrapTriggered
            | RoundResult::Evaded
            | RoundResult::Wounded => {
                // Choose a random section/sub-section tuple from our list of unvisited tuples
                let tup_ind = rand::thread_rng().gen_range(0, self.unvisited.len());

                // Get the tuple, remove it from the unvisited list and return it
                self.unvisited.remove(tup_ind)
            }

            // Special logic for a chase
            RoundResult::ChaseBegins(section) => {
                // Construct a vector of section/sub-section tuples that only contains
                // chase section tuples
                let mut valid_moves = Vec::<(usize, usize)>::new();
                for tup in &self.unvisited {
                    // Check if it's in the valid section
                    if tup.0 == section {
                        valid_moves.push(*tup);
                    }
                }

                // Special case if we have visited all the sections already
                if valid_moves.is_empty() {
                    valid_moves.push((
                        section,
                        rand::thread_rng().gen_range(0, state.sections[section].sub_sections.len()),
                    ));
                }

                // Choose a random move from that list
                let tup = valid_moves[rand::thread_rng().gen_range(0, valid_moves.len())];

                // Remove the move from the unvisted list
                let mut tup_to_remove = self.unvisited.len();
                for (i, unvisted) in self.unvisited.iter().enumerate() {
                    if *unvisted == tup {
                        tup_to_remove = i;
                        break;
                    }
                }

                // Only remove from unvisited if we are visiting a new place
                if tup_to_remove != self.unvisited.len() {
                    self.unvisited.remove(tup_to_remove);
                }

                tup
            }

            // Invalid round type
            _ => panic!("Invalid round type detected by victim AI."),
        };

        // If the move we are going to make results in us finding a part, we can remove all
        // section/sub-section tuples that are in the same section we are searching
        {
            if state.sections[tup.0].sub_sections[tup.1].part {
                // To do this, we take advantage of the guarantee made during construction of the
                // victim ai: the unvisited tuples are sorted by section. This means we can loop
                // over the unvisted tuples, marking the first and last tuples we see that are in
                // the correct section. Then, we can drain that range.

                // Find index of first and last valid section
                let mut section_range: (usize, usize) =
                    (self.unvisited.len(), self.unvisited.len());
                for (i, unvisited) in self.unvisited.iter().enumerate() {
                    // Check if we care about this tuple
                    if unvisited.0 == tup.0 {
                        // Mark beginning and end if beginning is unmarked
                        if section_range.0 == self.unvisited.len() {
                            section_range.0 = i;
                            section_range.1 = i;
                        }
                        // Mark end for the rest
                        else {
                            section_range.1 = i;
                        }
                    }
                }

                // Drain the elements if needed
                if section_range.0 == self.unvisited.len() {
                    self.unvisited.drain(section_range.0..section_range.1);
                }
            }
        }

        tup
    }
}
