use crate::lexer::Token;

#[derive(Debug)]
pub enum Command {
    Log(String),
    Scrape { url: String, selector: String },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current_token(&self) -> &Token {
        if self.position < self.tokens.len() {
            &self.tokens[self.position]
        } else {
            &Token::Eof
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();

        while self.current_token() != &Token::Eof {
            match self.current_token() {
                Token::Keyword(kw) if kw == "log" => {
                    self.advance();
                    if let Token::StringLiteral(msg) = self.current_token() {
                        commands.push(Command::Log(msg.clone()));
                        self.advance();
                    }
                }
                Token::Keyword(kw) if kw == "scrape" => {
                    self.advance();
                    let url = if let Token::StringLiteral(u) = self.current_token() {
                        let val = u.clone();
                        self.advance();
                        val
                    } else {
                        String::new()
                    };

                    let mut selector = String::new();

                    if self.current_token() == &Token::Symbol('{') {
                        self.advance();
                        if let Token::Identifier(id) = self.current_token() {
                            if id == "selector" {
                                self.advance();
                                if self.current_token() == &Token::Symbol(':') {
                                    self.advance();
                                    if let Token::StringLiteral(sel) = self.current_token() {
                                        selector = sel.clone();
                                        self.advance();
                                    }
                                }
                            }
                        }
                        if self.current_token() == &Token::Symbol('}') {
                            self.advance();
                        }
                    }

                    commands.push(Command::Scrape { url, selector });
                }
                _ => {
                    self.advance();
                }
            }
        }

        commands
    }
}
