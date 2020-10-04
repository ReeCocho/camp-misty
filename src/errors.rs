/// Different kinds of errors that might occur.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error
{
    OutOfBounds,

    NotFound
}