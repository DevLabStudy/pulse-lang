use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub extensions: Vec<String>,
    pub name_filters: Vec<String>,
    pub create_folder: Option<String>,
    pub move_to: Option<String>,
}

#[derive(Debug)]
pub struct Program {
    pub source_dir: String,
    pub rules: Vec<Rule>,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut source_dir = String::new();
        let mut rules = Vec::new();

        while self.current_token != Token::Eof {
            match &self.current_token {
                Token::Source => {
                    self.advance();
                    if let Token::String(path) = &self.current_token {
                        source_dir = path.clone();
                    }
                    self.advance();
                }
                Token::Rule => {
                    rules.push(self.parse_rule());
                }
                _ => {
                    self.advance();
                }
            }
        }

        Program { source_dir, rules }
    }

    fn parse_rule(&mut self) -> Rule {
        // Składnia: rule "Nazwa" { ... }
        self.advance(); // pomiń "rule"
        let name = match &self.current_token {
            Token::String(n) => n.clone(),
            _ => "Unnamed".to_string(),
        };
        self.advance(); // pomiń nazwę

        if self.current_token == Token::LBrace {
            self.advance();
        }

        let mut extensions = Vec::new();
        let mut name_filters = Vec::new();
        let mut create_folder = None;
        let mut move_to = None;

    	while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            match &self.current_token {
                Token::Match => {
                    self.advance();
                    if self.current_token == Token::Extension {
                        self.advance();
                        extensions = self.parse_string_or_list();
                    } else if self.current_token == Token::NameContains {
                        self.advance();
                        name_filters = self.parse_string_or_list();
                    }
                }
                Token::CreateFolder => {
                    self.advance();
                    if let Token::String(path) = &self.current_token {
                        create_folder = Some(path.clone());
                    }
                    self.advance();
                }
                Token::Move => {
                    self.advance();
                    if self.current_token == Token::To {
                        self.advance();
                        if let Token::String(path) = &self.current_token {
                            move_to = Some(path.clone());
                        }
                        self.advance();
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }

        if self.current_token == Token::RBrace {
            self.advance();
        }

        Rule {
            name,
            extensions,
            name_filters,
            create_folder,
            move_to,
        }
    }

    fn parse_string_or_list(&mut self) -> Vec<String> {
        let mut results = Vec::new();

        match &self.current_token {
            Token::String(s) => {
                results.push(s.clone());
                self.advance();
            }
            Token::LBracket => {
                self.advance();
                while self.current_token != Token::RBracket && self.current_token != Token::Eof {
                    if let Token::String(s) = &self.current_token {
                        results.push(s.clone());
                    }
                    self.advance();
                    if self.current_token == Token::Comma {
                        self.advance();
                    }
                }
                if self.current_token == Token::RBracket {
                    self.advance();
                }
            }
            _ => {}
        }

        results
    }
}
