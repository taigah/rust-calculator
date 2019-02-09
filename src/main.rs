mod lexer;
mod parser;
use std::io::Write;

fn main() {
    println!("Calculatrice Simple");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Une erreur de lecture d'entrée est survenue");
        let tokens = lexer::tokenize(&input);
        if tokens.is_err() {
            let err = tokens.unwrap_err();
            println!("Lexing Error\n{}", err);
            continue;
        }
        let tokens = tokens.unwrap();
        parser::parse(tokens);
    }
}
