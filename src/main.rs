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

    let mut lexer = Lexer::new(source_code);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let commands = parser.parse();

    println!("--- WYKONANIE PROGRAMU PULSE ---");
    for cmd in commands {
        match cmd {
            parser::Command::Log(msg) => {
                println!("[LOG]: {}", msg);
            }
            parser::Command::Scrape { url, selector } => {
                println!("[SCRAPE]: Łączę się z adresem '{}' używając selektora '{}'", url, selector);
                // Tutaj w przyszłości dodamy prawdziwe pobieranie HTML przez sieć!
            }
        }
    }
}
