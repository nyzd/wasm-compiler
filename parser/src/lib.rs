use error::{CompilerError, ErrorBucket};

use crate::{
    ast::Statement,
    lexer::token::{Span, Token},
};

pub mod ast;
pub mod lexer;

pub struct Parser<'a> {
    /// Current token
    token: Token,

    /// Final Abstract Syntax Tree
    ast: Statement,

    /// Error Handler Ref
    error_bucket: &'a mut ErrorBucket<ParserError>,
}

pub struct ParserError {
    id: usize,
    title: String,
    line_snippet: String,
    description: String,
}

impl ParserError {
    pub fn new(title: String, description: String, span: &Span) -> Self {
        Self {
            id: 0x2,
            line_snippet: format!("{}:{}", span.line, span.col),
            title,
            description: description.to_string(),
        }
    }
}

impl CompilerError for ParserError {
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
        "Parser"
    }
}
