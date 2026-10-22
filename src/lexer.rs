#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Source,
    Rule,
    Match,
    Extension,
    NameContains,
    GroupByDate,
    CreateFolder,
    Move,
    To,
    Identifier(String),
    String(String),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
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

        match current {
            '{' => { self.position += 1; Token::LBrace }
            '}' => { self.position += 1; Token::RBrace }
            '[' => { self.position += 1; Token::LBracket }
            ']' => { self.position += 1; Token::RBracket }
            ',' => { self.position += 1; Token::Comma }
            '"' => self.read_string(),
            _ => {
                if current.is_alphabetic() || current == '_' {
                    self.read_identifier_or_keyword()
                } else {
                    self.position += 1;
                    self.next_token()
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_string(&mut self) -> Token {
        self.position += 1; // pomiń otwieraący cudzysłów
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
            "source" => Token::Source,
            "rule" => Token::Rule,
            "match" => Token::Match,
            "extension" => Token::Extension,
            "name_contains" => Token::NameContains,
            "group_by_date" => Token::GroupByDate,
            "create_folder" => Token::CreateFolder,
            "move" => Token::Move,
            "to" => Token::To,
            _ => Token::Identifier(word),
        }
    }
}
