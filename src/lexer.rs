#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Log,
    Scrape,
    Selector,
    Let,
    Assign,
    For,
    In,
    Save,
    To,
    Identifier(String),
    String(String),
    LBrace,
    RBrace,
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
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::Eof;
        }

        let current = self.input[self.position];

        if current == '=' {
            self.position += 1;
            return Token::Assign;
        }

        if current == '{' {
            self.position += 1;
            return Token::LBrace;
        }

        if current == '}' {
            self.position += 1;
            return Token::RBrace;
        }

        if current == '"' {
            return self.read_string();
        }

        if current.is_alphabetic() {
            return self.read_identifier_or_keyword();
        }

        self.position += 1;
        self.next_token()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_string(&mut self) -> Token {
        self.position += 1; // pomiń otwierający cudzysłów
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.position += 1;
        }
        let s: String = self.input[start..self.position].iter().collect();
        if self.position < self.input.len() {
            self.position += 1; // pomiń zamykający cudzysłów
        }
        Token::String(s)
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len()
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            self.position += 1;
        }
        let word: String = self.input[start..self.position].iter().collect();

        match word.as_str() {
            "log" => Token::Log,
            "scrape" => Token::Scrape,
            "selector" => Token::Selector,
            "let" => Token::Let,
            "for" => Token::For,
            "in" => Token::In,
            "save" => Token::Save,
            "to" => Token::To,
            _ => Token::Identifier(word),
        }
    }
}
