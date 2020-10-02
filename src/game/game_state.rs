use crate::game::Section;
use crate::game::SubSection;
use crate::game::CarPart;

/// Total number of sections in the game.
pub const SECTION_COUNT : usize = 5;

/// Structure describing the current state of the game.
pub struct GameState
{
    // Sections within the game.
    pub sections : [Section; SECTION_COUNT],

    /// Type of round to occur next
    pub round_type : RoundType
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
                SubSection::new(String::from("(S)outh shore)"), 'S', CarPart::None)   
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
        GameState
        {
            sections : 
            [
                cabin,
                lake_misty,
                abandoned_manor,
                bonfire,
                old_forest
            ],
            round_type : RoundType::Normal
        }
    }

    /// Hide a car part in a sub section by index
    pub fn hide_part(&mut self, section : usize, sub_section : usize, part : CarPart)
    {
        self.sections[section].sub_sections[sub_section].part = part;
    }

    /// Perform a round in the game
}



/// A type of round in the game
#[derive(PartialEq)]
pub enum RoundType
{
    /// A normal round where the victim and killer choose a section and subsection to search.
    Normal,

    /// A chase round where the victim and killer are limited to a single section.
    ///
    /// Includes the index of the section where the chase is occuring.
    Chase(usize)
}

/// A result of a round in the game
#[derive(PartialEq)]
pub enum RoundResult
{
    /// Nothing happens.
    Nothing,

    /// The victim and killer chose the same section, beginning a chase.

    /// The killer caught the victim.
    KillerWin,

    /// The victim found all the parts.
    VictimWin,

    /// The victim evaded the killer during a chase.
    Evaded,

    /// The victim found a part.
    /// 
    /// Includes which part was found
    PartFound(CarPart),
}