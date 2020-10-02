/// A sub-section contained within a section.
pub struct SubSection
{
    /// Name of the sub-section.
    pub name : String,

    // Letter identifying the sub section.
    pub letter : char,

    // Part contained within the sub-section.
    pub part : CarPart
}

impl SubSection
{
    /// Constructs a sub section given a `name`, `letter`, and `part`.
    /// 
    /// NOTE: `letter` should be an uppercase letter.
    pub fn new(name : String, letter : char, part : CarPart) -> SubSection
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

/// A car part the victim might be looking for.
#[derive(Clone, Copy, PartialEq)]
pub enum CarPart
{
    None,
    Gasoline,
    Battery,
    SparkPlug,
    Headlights
}