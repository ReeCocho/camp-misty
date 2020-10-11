use crate::game::sub_section::*;

/// Total number of sub-sections contained within a section.
pub const SUB_SECTION_COUNT: usize = 5;

/// A section which contains SUB_SECTION_COUNT sub-sections the victim and killer might search.
pub struct Section {
    /// Name of the section.
    pub name: String,

    // Letter identifying the section.
    pub letter: char,

    // Sub-sections.
    pub sub_sections: [SubSection; SUB_SECTION_COUNT],

    /// A flag indicating if the section is trapped or not.
    pub trapped: bool,
}

impl Section {
    /// Constructs a section given a `name`, `letter`, and array of `sub_sections`.
    ///
    /// NOTE: `letter` should be an uppercase letter.
    pub fn new(
        name: String,
        letter: char,
        sub_sections: [SubSection; SUB_SECTION_COUNT],
    ) -> Section {
        assert_eq!(letter.is_alphabetic(), true);
        assert_eq!(letter.is_uppercase(), true);

        Section {
            name,
            letter,
            sub_sections,
            trapped: false,
        }
    }
}
