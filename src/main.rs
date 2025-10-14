use parser::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("12345 999999 \"Bruh\" 123456 let bruh +-/*()[] 0b @");
    let iter = lexer.into_iter();

    for (tkn, err) in iter {
        println!("{:?}", tkn);
        if let Some(err) = err {
            println!("{}", err);
        }
    }
}
