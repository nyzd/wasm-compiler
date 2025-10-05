pub mod token;

use std::str::Chars;
use std::iter::Peekable;

use crate::lexer::token::Token;

pub struct Lexer<'a> {
    input_chars: Peekable<Chars<'a>>,
}

impl <'a>Lexer<'a>{
    pub fn new(input: &'a str) -> Self {
        Self {
            input_chars: input.chars().peekable(),
        }
    }


    fn lex(&mut self) -> Token {
        let Some(next) = self.input_chars.next() else {
            return Token::Eof;
        };

        match next {
            '"' => {
                self.lex_string()
            }

            '0'..='9' => {
                self.lex_number(next)
            },

            _ => Token::Illegal
        }
    }

    fn lex_number(&mut self, current: char) -> Token {
        let mut final_num = String::new();
        final_num.push(current);

        while let Some(ch) = self.input_chars.peek() && ch.is_numeric() {
            final_num.push(*ch);
            self.input_chars.next();
        }

        // TODO: Handle parse Result
        Token::Number(final_num.parse().unwrap())
    }

    fn lex_string(&mut self) -> Token {
        let mut final_str = String::new();

        while let Some(ch) = self.input_chars.next() && ch != '"' {
            final_str.push(ch);
        }

        Token::String(final_str)
    }
}

impl <'a>Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex();

        if token == Token::Eof {
            return None;
        }

        Some(token)
    }
}