pub mod token;

use error::{CompilerError, ErrorBucket, ErrorReporter};
use std::fmt::Display;
use std::iter::Peekable;
use std::str::Chars;

use crate::lexer::token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    input_chars: Peekable<Chars<'a>>,
    errors: Vec<LexerError>,
    error_bucket: &'a mut ErrorBucket<LexerError>,
    line: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, error_bucket: &'a mut ErrorBucket<LexerError>) -> Self {
        Self {
            input_chars: input.chars().peekable(),
            errors: vec![],
            error_bucket,
            line: 1,
            col: 1,
        }
    }

    fn lex(&mut self) -> Token {
        let Some(next) = self.input_chars.next() else {
            return Token::new(TokenKind::Eof, Span::new(self.line, self.col));
        };

        match next {
            '0'..='9' => self.lex_number(next),
            'a'..='z' | 'A'..='Z' => self.lex_keyword_or_ident(next),

            // Ignore empty space
            ' ' => {
                self.col += 1;
                self.lex()
            }
            '\n' | '\r' => {
                self.line += 1;
                self.lex()
            }

            c => {
                let (line, col) = (self.line, self.col);
                self.col += 1;
                Token::new(TokenKind::from(c), Span::new(line, col))
            }
        }
    }

    fn lex_number(&mut self, current_char: char) -> Token {
        let mut final_num = String::from(current_char);
        let (line, col) = (self.line, self.col);
        self.col += 1;

        while let Some(ch) = self.input_chars.peek()
            && ch.is_numeric()
        {
            final_num.push(*ch);
            self.col += 1;
            self.input_chars.next();
        }

        let parsed = final_num.parse::<usize>().unwrap_or_else(|_| {
            self.report(LexerError::new(
                "Couldn't Parse the number!".to_string(),
                "ERRR".to_string(),
                &Span::new(line, col),
            ));
            0
        });

        Token::new(TokenKind::Number(parsed), Span::new(line, col))
    }

    fn lex_keyword_or_ident(&mut self, current_char: char) -> Token {
        let mut ident_string = String::from(current_char);
        let (line, col) = (self.line, self.col);
        self.col += 1;

        while let Some(ch) = self.input_chars.peek()
            && ch.is_alphabetic()
        {
            ident_string.push(*ch);
            self.col += 1;
            self.input_chars.next();
        }

        Token::new(TokenKind::from(ident_string), Span::new(line, col))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex();

        if token.kind == TokenKind::Eof {
            return None;
        }

        if let TokenKind::Illegal(c) = token.kind {
            self.error_bucket.report(LexerError::new(
                "Illegal char".to_string(),
                format!("Couldn't lex this char '{}'", c),
                &token.span,
            ));
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
    // Maybe can use Cow<Span>
    pub fn new(title: String, description: String, span: &Span) -> Self {
        Self {
            id: 0x1,
            line_snippet: format!("{}:{}", span.line, span.col),
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
