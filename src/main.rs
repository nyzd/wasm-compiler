use parser::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("12345 999999 \"Bruh\" 123456 let bruh +-/*()[]");
    let iter = lexer.into_iter();

    for tkn in iter {
        println!("{:?}", tkn);
    }
}
