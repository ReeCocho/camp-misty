use rand::seq::SliceRandom;
use rand::Rng;

use crate::game::section::*;
use crate::game::sub_section::*;

/// Total number of sections in the game.
pub const SECTION_COUNT : usize = 5;

/// Structure describing the current state of the game.
pub struct GameState
{
    // Sections within the game.
    pub sections : [Section; SECTION_COUNT],

    /// Result of the last round played. Also contains the index
    /// of the section the car part was found at. If no part was
    /// found, it will be equal to SECTION_COUNT.
    pub last_result : (RoundResult, usize),

    /// Total number of hidden parts
    pub part_count : usize,

    /// Flag indicating the victim is wounded
    pub victim_is_wounded : bool
}

impl GameState
{
    /// Constructor.
    pub fn new() -> GameState
    {
        // Create sections
        let cabin = Section::new(
            String::from("(C)abin"), 
            'C',
            [
                SubSection::new(String::from("(B)edroom"), 'B', CarPart::None),
                SubSection::new(String::from("(K)itchen"), 'K', CarPart::None),
                SubSection::new(String::from("(T)oilet"), 'T', CarPart::None),
                SubSection::new(String::from("(C)loset"), 'C', CarPart::None),
                SubSection::new(String::from("(A)ttic"), 'A', CarPart::None)   
            ]);

        let lake_misty = Section::new(
            String::from("(L)ake Misty"), 
            'L',
            [
                SubSection::new(String::from("(D)ock"), 'D', CarPart::None),
                SubSection::new(String::from("(B)oat"), 'B', CarPart::None),
                SubSection::new(String::from("(E)ast shore"), 'E', CarPart::None),
                SubSection::new(String::from("(W)est shore"), 'W', CarPart::None),
                SubSection::new(String::from("(S)outh shore"), 'S', CarPart::None)   
            ]);

        let abandoned_manor = Section::new(
            String::from("(A)bandoned manor"), 
            'A',
            [
                SubSection::new(String::from("(M)aster bedroom"), 'M', CarPart::None),
                SubSection::new(String::from("(D)ining hall"), 'D', CarPart::None),
                SubSection::new(String::from("(B)asement"), 'B', CarPart::None),
                SubSection::new(String::from("(K)itchen"), 'K', CarPart::None),
                SubSection::new(String::from("(F)ourier"), 'F', CarPart::None)   
            ]);

        let bonfire = Section::new(
            String::from("(B)onfire"), 
            'B',
            [
                SubSection::new(String::from("(S)hrubs"), 'S', CarPart::None),
                SubSection::new(String::from("(C)ouch"), 'C', CarPart::None),
                SubSection::new(String::from("(L)ogs"), 'L', CarPart::None),
                SubSection::new(String::from("(T)rees"), 'T', CarPart::None),
                SubSection::new(String::from("(B)lankets"), 'B', CarPart::None)   
            ]);

        let old_forest = Section::new(
            String::from("(O)ld forest"), 
            'O',
            [
                SubSection::new(String::from("(P)ond"), 'P', CarPart::None),
                SubSection::new(String::from("(C)ave"), 'C', CarPart::None),
                SubSection::new(String::from("(S)hrine"), 'S', CarPart::None),
                SubSection::new(String::from("(F)airy circle"), 'F', CarPart::None),
                SubSection::new(String::from("(H)ollow log"), 'H', CarPart::None)   
            ]);
        
        // Construct game state
        let mut state = GameState
        {
            sections : 
            [
                cabin,
                lake_misty,
                abandoned_manor,
                bonfire,
                old_forest
            ],
            last_result : (RoundResult::Nothing, SECTION_COUNT),
            part_count : 0,
            victim_is_wounded : false
        };

        // Take a list of all car parts and shuffle them
        let car_parts = [
            CarPart::Battery,
            CarPart::Headlights,
            CarPart::SparkPlug,
            CarPart::Gasoline];
        
        // Create a list of indices
        let mut part_loc_inds : Vec<usize> = (0..SECTION_COUNT).collect();

        // Remove one random index from the list (this indicates which section doesn't have a part)
        part_loc_inds.remove(rand::thread_rng().gen_range(0, part_loc_inds.len()));

        // Shuffle the index. Then, we can distribute the parts randomly among the sections
        part_loc_inds.shuffle(&mut rand::thread_rng());

        // Distribute car parts
        for (part_index, section_index) in part_loc_inds.into_iter().enumerate()
        {
            // Randomly choose which sub section gets the part
            let rand_ind = rand::thread_rng().gen_range(0, SUB_SECTION_COUNT);

            // Place the part in the sub section
            state.hide_part(section_index, rand_ind, car_parts[part_index]);
        }

        // Return new game state
        return state;
    }

    /// Hide a car part in a sub section by index.
    pub fn hide_part(&mut self, section : usize, sub_section : usize, part : CarPart)
    {
        self.sections[section].sub_sections[sub_section].part = part;
        self.part_count += 1;
    }

    /// Place a trap in a section.
    pub fn place_trap(&mut self, section : usize)
    {
        // Place the trap
        self.sections[section].trapped = true;
    }

    /// Determine if a section of the map is trapped.
    pub fn trap_exists(&self) -> bool
    {
        // Loop over all sections
        for section in &self.sections
        {
            // Check if the section is trapped
            if section.trapped
            {
                return true;
            }
        }

        // No section was found that is trapped
        return false;
    }

    /// Get the index of a section by its letter identifier.
    pub fn get_section_by_letter(&self, id : char) -> Option<usize>
    {
        // Loop over all sections
        for (i, section) in self.sections.iter().enumerate()
        {
            // If the letter matches, choose that one
            if section.letter == id
            {
                return Some(i);
            }
        }

        // Didn't find a section
        return None;
    }

    /// Get the index of a sub section by its letter identifier.
    /// 
    /// The first argument is the index of the section to check and the second argument is the letter of the sub section.
    pub fn get_sub_section_by_letter(&self, section : usize, id : char) -> Option<usize>
    {
        // Out of bounds section
        if section >= SECTION_COUNT
        {
            return None;
        }

        // Loop over all sub sections in the section
        for (i, sub_section) in self.sections[section].sub_sections.iter().enumerate()
        {
            // If the letter matches, choose that one
            if sub_section.letter == id
            {
                return Some(i);
            }
        }

        // Didn't find a sub section
        return None;
    }

    /// Get a tuple containing the index of a section and sub-section respectively by letter identifier.
    /// 
    /// If the indices were not found, will return None.
    pub fn get_inds_by_letter(&self, section_char : char, sub_section_char : char) -> Option<(usize, usize)>
    {
        // Loop over every section
        for (i, section) in self.sections.iter().enumerate()
        {
            // If the section is the one we are looking for...
            if section.letter == section_char
            {
                // Loop over all sub sections in the section
                for (j, sub_section) in self.sections.iter().enumerate()
                {
                    // If the sub section is the one we are looking for...
                    if sub_section.letter == sub_section_char
                    {
                        // Return that section/sub-section pair
                        return Some((i, j));
                    }
                }
            }
        }

        // Couldn't find the section/sub-section pair
        return None;
    }

    /// Perform a round of the game.
    /// 
    /// `victim` is a tuple containing the indices of the section and sub-section the victim is checking.
    /// 
    /// `killer` is a tuple containing the indices of the section and sub-section the victim is checking.
    /// 
    /// Returns a `PlayResult` with either an OutOfBounds error, or a tuple containing the result of the round and
    /// the car part found by the player (may be None).
    pub fn play(&mut self, victim : (usize, usize), killer : (usize, usize)) -> PlayResult
    {
        // Indices must be within bounds
        if  victim.0 >= SECTION_COUNT || 
            victim.1 >= SUB_SECTION_COUNT || 
            killer.0 >= SECTION_COUNT || 
            killer.1 >= SUB_SECTION_COUNT
        {
            return Err(PlayError::OutOfBounds);
        }

        // Special check for chase round
        match self.last_result.0
        {
            // Victim and killer must choose the section that the chase is taking place in
            RoundResult::ChaseBegins(section) =>
            if victim.0 != section || killer.0 != section
            {
                return Err(PlayError::OutOfBounds);
            }

            // Ignore all other cases
            _ => {}
        }

        // Get the car part in the section
        let car_part = self.sections[victim.0].sub_sections[victim.1].part;

        // Get rid of part if needed
        if car_part != CarPart::None
        {
            // Remove the part
            self.sections[victim.0].sub_sections[victim.1].part = CarPart::None;

            // Decrement part count
            self.part_count -= 1;
        }

        // Hold the result of the round
        let mut round_result = RoundResult::Nothing;

        // If the killer chooses a trapped section, the killer is trapped
        if self.sections[killer.0].trapped
        {
            round_result = RoundResult::TrapTriggered;
        }

        // If the victim found all parts, they win (priority over trapping)
        if self.part_count == 0
        {
            round_result = RoundResult::AllPartsFound;
        }
        
        // If the killer and the victim chose the same exact place, and the killer isn't trapped, the killer wins
        if  self.last_result.0 != RoundResult::TrapTriggered && 
            victim.0 == killer.0 && 
            victim.1 == killer.1
        {
            // If the victim has already been wounded, they are caught (priority over winning)
            if self.victim_is_wounded
            {
                round_result = RoundResult::Caught;
            }
            // If the victim hasn't been wounded yet, wound them (unless the victim is winning, in which case do nothing)
            else if self.part_count != 0
            {
                self.victim_is_wounded = true;
                round_result = RoundResult::Wounded;
            }
        }
        // If a chase was occuring, the victim evaded (ignore if player is winning)
        else if self.part_count > 0 && 
                std::mem::discriminant(&self.last_result.0) == std::mem::discriminant(&RoundResult::ChaseBegins(0))
        {
            round_result = RoundResult::Evaded;
        }
        // If the victim and killer chose the same section, a chase begins (priority over player winning)
        else if victim.0 == killer.0
        {
            round_result = RoundResult::ChaseBegins(victim.0);
        }

        // Update result
        self.last_result = (round_result.clone(), if car_part != CarPart::None { victim.0 } else { SECTION_COUNT });

        return Ok((round_result, (car_part, victim.0)));
    }
}



/// A result of a round in the game
#[derive(Debug, PartialEq, Clone)]
pub enum RoundResult
{
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
    TrapTriggered
}



/// Types of errors to occur during play.
#[derive(Debug)]
pub enum PlayError
{
    /// Some index was out of bounds
    OutOfBounds
}

/// Result type of playing a round of the game.
/// 
/// Contains the round result and car part found including what section it was found in.
pub type PlayResult = std::result::Result<(RoundResult, (CarPart, usize)), PlayError>;



/// Testing for game state.
#[cfg(test)]
mod test
{
    use crate::game::killer_ai::*;
    use crate::game::victim_ai::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Runs a simulation of the game with AI players.
    #[test]
    fn simulation()
    {
        // Number of simulated games to play
        const GAME_COUNT : usize = 1000;

        // Play matches
        for _ in 0..GAME_COUNT
        {
            // Create a game state
            let state = Rc::new(RefCell::new(super::GameState::new()));

            // Create AI plays
            let mut killer = KillerAI::new(state.clone());
            let mut victim = VictimAI::new(state.clone());

            // Play game until there is a winner
            loop
            {
                // Have each AI make a move
                let killer_move = killer.play();
                let victim_move = victim.play();

                // Submit moves to the game state
                let res = state.borrow_mut().play(victim_move, killer_move).expect("Something went wrong during play");

                // Place trap if evaded
                if res.0 == super::RoundResult::Evaded
                {
                    victim.place_trap();
                }
                // Break if someone won
                else if res.0 == super::RoundResult::Caught
                {
                    break;
                }
                else if res.0 == super::RoundResult::AllPartsFound
                {
                    break;
                }
            }
        }
    }
}