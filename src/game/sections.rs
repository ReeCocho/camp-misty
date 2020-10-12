/// A section which contains sub-sections the victim and killer might search.
pub struct Section {
    /// Name of the section.
    pub name: String,

    // Letter identifying the section.
    pub letter: char,

    // Sub-sections.
    pub sub_sections: Vec<SubSection>,

    /// A flag indicating if the section is trapped or not.
    pub trapped: bool,
}

/// A sub-section contained within a section.
pub struct SubSection {
    /// Name of the sub-section.
    pub name: String,

    // Letter identifying the sub section.
    pub letter: char,

    // If a part is contained within the sub-section.
    pub part: bool,
}

impl Section {
    /// Constructs a section given a `name`, `letter`, and array of `sub_sections`.
    ///
    /// NOTE: `letter` should be an uppercase letter.
    pub fn new(name: String, letter: char, sub_sections: Vec<SubSection>) -> Section {
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

impl SubSection {
    /// Constructs a sub section given a `name`, `letter`, and optional `part`.
    ///
    /// NOTE: `letter` should be an uppercase letter.
    pub fn new(name: String, letter: char, part: bool) -> SubSection {
        assert_eq!(letter.is_alphabetic(), true);
        assert_eq!(letter.is_uppercase(), true);

        SubSection { name, letter, part }
    }
}
