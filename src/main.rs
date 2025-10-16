use error::{ErrorBucket, ErrorReporter};
use parser::lexer::{Lexer, token::Token};

fn main() {
    let mut lexer_error_bucket = ErrorBucket::new();
    let lexer = Lexer::new(
        "12345 999999 \"Bruh\" 123456 let bruh +-/*()[] 0b @",
        &mut lexer_error_bucket,
    );
    let iter: Vec<Token> = lexer.into_iter().collect();

    for tkn in iter {
        println!("{:?}", tkn);
    }

    for err in lexer_error_bucket.errors() {
        println!("{}", err);
    }
}
