mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::Parser;

fn main() {
    let source_code = r#"
        log "Startuje silnik Pulse..."
        scrape "https://sklep.pl/produkt" {
            selector: ".price"
        }
    "#;

    // 1. Faza Leksera
    let mut lexer = Lexer::new(source_code);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    // 2. Faza Parsera
    let mut parser = Parser::new(tokens);
    let commands = parser.parse();

    println!("--- WYNIK PARSOWANIA (AST) ---");
    for cmd in commands {
        println!("{:?}", cmd);
    }
}
