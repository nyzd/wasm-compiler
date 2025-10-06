#[derive(PartialEq, Eq, Debug)]
pub enum Keyword {
    Let,
    Function,
    Return,
    If,
    Else,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    /// End of file
    Eof,

    /// Not supported char
    Illegal(char),

    // Symbols
    /// =
    Equal,

    /// +
    Plus,

    /// -
    Minus,

    /// *
    Asterisk,

    /// '/'
    Slash,

    /// '.' (full stop)
    Dot,

    /// ':'
    Colon,

    /// ';'
    Semicolon,

    /// <
    Lessthan,

    /// <
    Greaterthan,

    /// [
    LeftSquareBracket,

    /// ]
    RightSquareBracket,

    /// (
    LeftParent,

    /// )
    RightParent,

    /// "
    Quotation,

    /// Simple Number
    Number(usize),

    /// Identifier ex. variable or function name
    Identifier(String),

    Keyword(Keyword)
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "let" => Self::Keyword(Keyword::Let),
            "fn" => Self::Keyword(Keyword::Function),
            "return" => Self::Keyword(Keyword::Return),
            "if" => Self::Keyword(Keyword::If),
            "else" => Self::Keyword(Keyword::Else),

            other => Self::Identifier(other.to_string()),
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '=' => Self::Equal,
            '<' => Self::Lessthan,
            '>' => Self::Greaterthan,
            ':' => Self::Colon,
            ';' => Self::Semicolon,
            '(' => Self::LeftParent,
            ')' => Self::RightParent,
            '[' => Self::LeftSquareBracket,
            ']' => Self::RightSquareBracket,
            '.' => Self::Dot,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Asterisk,
            '/' => Self::Slash,
            '"' => Self::Quotation,

            c => Self::Illegal(c)
        }
    }
}