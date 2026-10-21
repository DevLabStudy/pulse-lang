#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    StringLiteral(String),
    Symbol(char),
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.position < self.input.len() {
            let current_char = self.input[self.position];

            if current_char.is_whitespace() {
                self.position += 1;
                continue;
            }

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

            if current_char == '"' {
                self.position += 1;
                let start = self.position;
                while self.position < self.input.len() && self.input[self.position] != '"' {
                    self.position += 1;
                }
                let string_val: String = self.input[start..self.position].iter().collect();
                if self.position < self.input.len() {
                    self.position += 1;
                }
                return Token::StringLiteral(string_val);
            }

            if current_char == '{' || current_char == '}' || current_char == ':' {
                self.position += 1;
                return Token::Symbol(current_char);
            }

            self.position += 1;
        }

        Token::Eof
    }
}
