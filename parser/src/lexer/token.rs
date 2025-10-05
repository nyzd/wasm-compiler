#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    /// End of file
    Eof,

    /// Not supported char
    Illegal,

    /// Simple Number
    Number(usize),

    /// Simple string
    String(String),
}
