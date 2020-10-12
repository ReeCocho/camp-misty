use rand::Rng;

use crate::game::sections::*;

/// Structure describing the current state of the game.
pub struct GameState {
    // Sections within the game.
    pub sections: Vec<Section>,

    /// Result of the last round played.
    pub last_result: PlayResult,

    /// Total number of hidden parts
    pub part_count: usize,

    /// Flag indicating the victim is wounded
    pub victim_is_wounded: bool,
}

/// The result of a previous round and an optional car part if one was found.
#[derive(Copy, Clone)]
pub struct PlayResult {
    /// Result of the round.
    pub result: RoundResult,

    /// Index of the part (if found).
    pub part_section_index: Option<usize>,
}

/// A result of a round in the game
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RoundResult {
    /// Nothing happens.
    Nothing,

    /// The killer caught the victim.
    Caught,

    /// The killer wounded the victim.
    Wounded,

    /// The victim found all the parts.
    AllPartsFound,

    /// The victim evaded the killer during a chase.
    Evaded,

    /// The victim and killer chose the same section, beginning a chase.
    ///
    /// Includes the index of the section where the chase is going to take place.
    ChaseBegins(usize),

    /// The killer triggered a trap.
    TrapTriggered,
}

impl Default for PlayResult {
    fn default() -> Self {
        PlayResult::new(RoundResult::Nothing, None)
    }
}

impl PlayResult {
    pub fn new(result: RoundResult, part_section_index: Option<usize>) -> Self {
        PlayResult {
            result,
            part_section_index,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Constructor.
    pub fn new() -> GameState {
        // Create sections
        let sections = vec![
            Section::new(
                String::from("(C)abin"),
                'C',
                vec![
                    SubSection::new(String::from("(B)edroom"), 'B', false),
                    SubSection::new(String::from("(K)itchen"), 'K', false),
                    SubSection::new(String::from("(T)oilet"), 'T', false),
                    SubSection::new(String::from("(C)loset"), 'C', false),
                    SubSection::new(String::from("(A)ttic"), 'A', false),
                ],
            ),
            Section::new(
                String::from("(L)ake Misty"),
                'L',
                vec![
                    SubSection::new(String::from("(D)ock"), 'D', false),
                    SubSection::new(String::from("(B)oat"), 'B', false),
                    SubSection::new(String::from("(E)ast shore"), 'E', false),
                    SubSection::new(String::from("(W)est shore"), 'W', false),
                    SubSection::new(String::from("(S)outh shore"), 'S', false),
                ],
            ),
            Section::new(
                String::from("(A)bandoned manor"),
                'A',
                vec![
                    SubSection::new(String::from("(M)aster bedroom"), 'M', false),
                    SubSection::new(String::from("(D)ining hall"), 'D', false),
                    SubSection::new(String::from("(B)asement"), 'B', false),
                    SubSection::new(String::from("(K)itchen"), 'K', false),
                    SubSection::new(String::from("(F)ourier"), 'F', false),
                ],
            ),
            Section::new(
                String::from("(B)onfire"),
                'B',
                vec![
                    SubSection::new(String::from("(S)hrubs"), 'S', false),
                    SubSection::new(String::from("(C)ouch"), 'C', false),
                    SubSection::new(String::from("(L)ogs"), 'L', false),
                    SubSection::new(String::from("(T)rees"), 'T', false),
                    SubSection::new(String::from("(B)lankets"), 'B', false),
                ],
            ),
            Section::new(
                String::from("(O)ld forest"),
                'O',
                vec![
                    SubSection::new(String::from("(P)ond"), 'P', false),
                    SubSection::new(String::from("(C)ave"), 'C', false),
                    SubSection::new(String::from("(S)hrine"), 'S', false),
                    SubSection::new(String::from("(F)airy circle"), 'F', false),
                    SubSection::new(String::from("(H)ollow log"), 'H', false),
                ],
            ),
        ];

        GameState {
            sections,
            last_result: PlayResult::new(RoundResult::Nothing, None),
            part_count: 0,
            victim_is_wounded: false,
        }
    }

    /// Generate random game state.
    pub fn gen_state(&mut self) {
        // Distribute car parts
        for i in 0..self.sections.len() {
            // Randomly choose which sub section gets the part
            let rand_ind = rand::thread_rng().gen_range(0, self.sections[i].sub_sections.len());

            // Place the part in the sub section
            self.hide_part(i, rand_ind);
        }
    }

    /// Hide a car part in a sub section by index.
    pub fn hide_part(&mut self, section: usize, sub_section: usize) {
        self.sections[section].sub_sections[sub_section].part = true;
        self.part_count += 1;
    }

    /// Get the index of a section by its letter identifier.
    pub fn get_section_by_letter(&self, id: char) -> Option<usize> {
        self.sections.iter().position(|s| s.letter == id)
    }

    /// Get the index of a sub section by its letter identifier.
    ///
    /// The first argument is the index of the section to check and the second argument is the letter of the sub section.
    pub fn get_sub_section_by_letter(&self, section: usize, id: char) -> Option<usize> {
        // Out of bounds section
        if section >= self.sections.len() {
            return None;
        }

        self.sections[section]
            .sub_sections
            .iter()
            .position(|s| s.letter == id)
    }

    /// Get a tuple containing the index of a section and sub-section respectively by letter identifier.
    ///
    /// If the indices were not found, will return None.
    pub fn get_inds_by_letter(
        &self,
        section_char: char,
        sub_section_char: char,
    ) -> Option<(usize, usize)> {
        // Loop over every section
        if let Some(i) = self.get_section_by_letter(section_char) {
            if let Some(j) = self.get_sub_section_by_letter(i, sub_section_char) {
                return Some((i, j));
            }
        }

        None
    }

    /// Perform a round of the game.
    ///
    /// `victim` is a tuple containing the indices of the section and sub-section the victim is checking.
    ///
    /// `killer` is a tuple containing the indices of the section and sub-section the victim is checking.
    pub fn play(&mut self, victim: (usize, usize), killer: (usize, usize)) -> PlayResult {
        // Indices must be within bounds
        assert!(
            victim.0 < self.sections.len()
                && victim.1 < self.sections[victim.0].sub_sections.len()
                && killer.0 < self.sections.len()
                && killer.1 < self.sections[killer.0].sub_sections.len()
        );

        // Special check for chase round
        if let RoundResult::ChaseBegins(section) = self.last_result.result {
            assert!(victim.0 == section && killer.0 == section);
        }

        // Get the car part in the section
        let car_part = self.sections[victim.0].sub_sections[victim.1].part;

        // Get rid of part if needed
        if car_part {
            self.sections[victim.0].sub_sections[victim.1].part = false;
            self.part_count -= 1;
        }

        // Default round result is nothing happens
        let mut round_result = RoundResult::Nothing;

        // If the victim found all parts, they win
        if self.part_count == 0 {
            round_result = RoundResult::AllPartsFound;
        }

        // If the killer and the victim chose the same exact place...
        if victim.0 == killer.0 && victim.1 == killer.1 {
            // If the victim has already been wounded, they are caught (priority over winning)
            if self.victim_is_wounded {
                round_result = RoundResult::Caught;
            }
            // If the victim hasn't been wounded yet, wound them (unless the victim is winning, in which case do nothing)
            else if round_result != RoundResult::AllPartsFound {
                self.victim_is_wounded = true;
                round_result = RoundResult::Wounded;
            }
        }
        // If a chase was occuring, the victim evaded (ignore if player is winning)
        else if round_result != RoundResult::AllPartsFound
            && std::mem::discriminant(&self.last_result.result)
                == std::mem::discriminant(&RoundResult::ChaseBegins(0))
        {
            round_result = RoundResult::Evaded;
        }
        // If the victim and killer chose the same section, a chase begins (priority over player winning)
        else if victim.0 == killer.0 && round_result != RoundResult::AllPartsFound {
            round_result = RoundResult::ChaseBegins(victim.0);
        }

        // Update last result
        let res = PlayResult::new(round_result, if car_part { Some(victim.0) } else { None });
        self.last_result = res;

        res
    }
}

/// Testing for game state.
#[cfg(test)]
mod test {
    use crate::game::killer_ai::*;
    use crate::game::victim_ai::*;

    /// Runs a simulation of the game with AI players.
    #[test]
    fn simulation() {
        // Number of simulated games to play
        const GAME_COUNT: usize = 10000;

        // Number of wins for each player type (used to compute win to loss ratio)
        let mut victim_wins: usize = 0;
        let mut killer_wins: usize = 0;

        // Play matches
        for _ in 0..GAME_COUNT {
            // Create a game state
            let mut state = super::GameState::new();
            state.gen_state();

            // Create AI plays
            let mut killer = KillerAI::new(&state);
            let mut victim = VictimAI::new(&state);

            // Play game until there is a winner
            loop {
                // Have each AI make a move
                let killer_move = killer.play(&mut state);
                let victim_move = victim.play(&mut state);

                // Submit moves to the game state
                let res = state.play(victim_move, killer_move);

                // Break if someone won
                if res.result == super::RoundResult::Caught {
                    killer_wins += 1;
                    break;
                } else if res.result == super::RoundResult::AllPartsFound {
                    victim_wins += 1;
                    break;
                }
            }
        }

        // Print out win to loss ratio
        println!(
            "V/K win ratio = {}",
            (victim_wins as f32) / (killer_wins as f32)
        );
    }
}
