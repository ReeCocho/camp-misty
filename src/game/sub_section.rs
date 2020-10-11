/// A sub-section contained within a section.
pub struct SubSection
{
    /// Name of the sub-section.
    pub name : String,

    // Letter identifying the sub section.
    pub letter : char,

    // If a part is contained within the sub-section.
    pub part : bool
}

impl SubSection
{
    /// Constructs a sub section given a `name`, `letter`, and optional `part`.
    /// 
    /// NOTE: `letter` should be an uppercase letter.
    pub fn new(name : String, letter : char, part : bool) -> SubSection
    {
        assert_eq!(letter.is_alphabetic(), true);
        assert_eq!(letter.is_uppercase(), true);

        SubSection 
        {
            name : name,
            letter : letter,
            part : part
        }
    }
}