use crate::lexer::token::Token;

#[derive(Debug)]
pub struct NumberExpression(usize);

#[derive(Debug)]
pub enum Expression {
    Number(NumberExpression),
}

impl From<Token> for Expression {
    fn from(value: Token) -> Self {
        match value {
            Token::Number(n) => Self::Number(NumberExpression(n)),

            _ => todo!(),
        }
    }
}

pub struct Ast {}
