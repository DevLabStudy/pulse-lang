#[derive(Debug, PartialEq)]
enum Token {
    Keyword(String),
    Identifier(String),
    StringLiteral(String),
    Symbol(char),
    Eof,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        while self.position < self.input.len() {
            let current_char = self.input[self.position];

            // Pomijanie białych znaków
            if current_char.is_whitespace() {
                self.position += 1;
                continue;
            }

            // Obsługa słów kluczowych i identyfikatorów (np. scrape, log)
            if current_char.is_alphabetic() {
                let start = self.position;
                while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
                    self.position += 1;
                }
                let word: String = self.input[start..self.position].iter().collect();
                if word == "scrape" || word == "log" {
                    return Token::Keyword(word);
                }
                return Token::Identifier(word);
            }

            // Obsługa ciągów tekstowych w cudzysłowie ("...")
            if current_char == '"' {
                self.position += 1; // pomijamy otwarcie
                let start = self.position;
                while self.position < self.input.len() && self.input[self.position] != '"' {
                    self.position += 1;
                }
                let string_val: String = self.input[start..self.position].iter().collect();
                if self.position < self.input.len() {
                    self.position += 1; // pomijamy zamknięcie
                }
                return Token::StringLiteral(string_val);
            }

            // Obsługa pojedynczych symboli (np. klamry, dwukropki)
            if current_char == '{' || current_char == '}' || current_char == ':' {
                self.position += 1;
                return Token::Symbol(current_char);
            }

            self.position += 1;
        }

        Token::Eof
    }
}

fn main() {
    let source_code = r#"
        log "Startuje silnik Pulse..."
        scrape "https://sklep.pl/produkt" {
            selector: ".price"
        }
    "#;

    let mut lexer = Lexer::new(source_code);
    println!("--- ANALIZA LEKSYKALNA (TOKENY) ---");
    
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        println!("{:?}", token);
    }
}
