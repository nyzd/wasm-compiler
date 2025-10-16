pub mod token;

use error::{CompilerError, ErrorBucket, ErrorReporter};
use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

use crate::lexer::token::Token;

pub struct Lexer<'a> {
    input_chars: Peekable<Chars<'a>>,
    errors: Vec<LexerError>,
    error_bucket: &'a mut ErrorBucket<LexerError>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, error_bucket: &'a mut ErrorBucket<LexerError>) -> Self {
        Self {
            input_chars: input.chars().peekable(),
            errors: vec![],
            error_bucket,
        }
    }

    fn lex(&mut self) -> Token {
        let Some(next) = self.input_chars.next() else {
            return Token::Eof;
        };

        match next {
            '0'..='9' => self.lex_number(next),
            'a'..='z' | 'A'..='Z' => self.lex_keyword_or_ident(next),

            // Ignore empty space
            ' ' | '\n' | '\r' => self.lex(),

            c => Token::from(c),
        }
    }

    fn lex_number(&mut self, current_char: char) -> Token {
        let mut final_num = String::from(current_char);

        while let Some(ch) = self.input_chars.peek()
            && ch.is_numeric()
        {
            final_num.push(*ch);
            self.input_chars.next();
        }

        let parsed = final_num.parse::<usize>().unwrap_or_else(|_| {
            self.report(LexerError::new(
                "Couldn't Parse the number!".to_string(),
                "ERRR".to_string(),
                0,
                1,
            ));
            0
        });

        // TODO: Handle parse Result
        Token::Number(parsed)
    }

    fn lex_keyword_or_ident(&mut self, current_char: char) -> Token {
        let mut ident_string = String::from(current_char);

        while let Some(ch) = self.input_chars.peek()
            && ch.is_alphabetic()
        {
            ident_string.push(*ch);
            self.input_chars.next();
        }

        Token::from(ident_string)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex();

        if let Token::Illegal(c) = token {
            self.error_bucket.report(LexerError::new(
                "Illegal char".to_string(),
                format!("Couldn't lex this char '{}'", c),
                0,
                0,
            ));
        }

        if token == Token::Eof {
            return None;
        }

        Some(token)
    }
}

#[derive(Clone)]
pub struct LexerError {
    id: usize,
    title: String,
    line_snippet: String,
    description: String,
}

impl LexerError {
    pub fn new(title: String, description: String, line: u32, col: u32) -> Self {
        Self {
            id: 0x1,
            line_snippet: format!("{}:{}", line, col),
            title,
            description: description.to_string(),
        }
    }
}

impl CompilerError for LexerError {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn summary(&self) -> &str {
        &self.line_snippet
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn stage(&self) -> &'static str {
        "Lexer"
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} at {} => {}",
            self.stage(),
            self.title(),
            self.summary(),
            self.description()
        )?;

        Ok(())
    }
}

impl<'a> ErrorReporter<LexerError> for Lexer<'a> {
    fn report(&mut self, error: LexerError) {
        self.errors.push(error);
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn errors(&self) -> &[LexerError] {
        &self.errors
    }
}
