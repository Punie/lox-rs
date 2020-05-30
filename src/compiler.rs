use crate::lexer::Scanner;

pub fn compile(source: &str) {
    let mut scanner = Scanner::init(source);

    while let Some(token) = scanner.next_token() {
        println!("{}", token);
    }
}
