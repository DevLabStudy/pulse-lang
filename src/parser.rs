use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Command {
    Log(Expr),
    Scrape { url: String, selector: String },
    Let { name: String, command: Box<Command> },
    For { variable: String, collection: String, body: Vec<Command> },
    Save { variable: String, filename: String },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();
        while self.position < self.tokens.len() {
            if let Some(cmd) = self.parse_command() {
                commands.push(cmd);
            } else {
                self.position += 1;
            }
        }
        commands
    }

    fn parse_command(&mut self) -> Option<Command> {
        let token = self.tokens.get(self.position)?;

        match token {
            Token::Save => {
                self.position += 1;
                let variable = match self.tokens.get(self.position)? {
                    Token::Identifier(id) => {
                        let v = id.clone();
                        self.position += 1;
                        v
                    }
                    _ => return None,
                };

                if let Some(Token::To) = self.tokens.get(self.position) {
                    self.position += 1;
                } else {
                    return None;
                }

                let filename = match self.tokens.get(self.position)? {
                    Token::String(s) => {
                        let f = s.clone();
                        self.position += 1;
                        f
                    }
                    _ => return None,
                };

                Some(Command::Save { variable, filename })
            }
            Token::For => {
                self.position += 1;
                let variable = match self.tokens.get(self.position)? {
                    Token::Identifier(id) => {
                        let v = id.clone();
                        self.position += 1;
                        v
                    }
                    _ => return None,
                };

                if let Some(Token::In) = self.tokens.get(self.position) {
                    self.position += 1;
                } else {
                    return None;
                }

                let collection = match self.tokens.get(self.position)? {
                    Token::Identifier(id) => {
                        let c = id.clone();
                        self.position += 1;
                        c
                    }
                    _ => return None,
                };

                let mut body = Vec::new();
                if let Some(Token::LBrace) = self.tokens.get(self.position) {
                    self.position += 1;
                    while self.position < self.tokens.len() {
                        if let Some(Token::RBrace) = self.tokens.get(self.position) {
                            self.position += 1;
                            break;
                        }
                        if let Some(cmd) = self.parse_command() {
                            body.push(cmd);
                        } else {
                            self.position += 1;
                        }
                    }
                }

                Some(Command::For { variable, collection, body })
            }
            Token::Let => {
                self.position += 1;
                let name = match self.tokens.get(self.position)? {
                    Token::Identifier(id) => {
                        let n = id.clone();
                        self.position += 1;
                        n
                    }
                    _ => return None,
                };

                if let Some(Token::Assign) = self.tokens.get(self.position) {
                    self.position += 1;
                } else {
                    return None;
                }

                let cmd = self.parse_command()?;
                Some(Command::Let {
                    name,
                    command: Box::new(cmd),
                })
            }
            Token::Log => {
                self.position += 1;
                let expr = match self.tokens.get(self.position)? {
                    Token::String(s) => {
                        let val = s.clone();
                        self.position += 1;
                        Expr::String(val)
                    }
                    Token::Identifier(id) => {
                        let val = id.clone();
                        self.position += 1;
                        Expr::Variable(val)
                    }
                    _ => return None,
                };
                Some(Command::Log(expr))
            }
            Token::Scrape => {
                self.position += 1;
                let url = match self.tokens.get(self.position)? {
                    Token::String(s) => {
                        let u = s.clone();
                        self.position += 1;
                        u
                    }
                    _ => return None,
                };

                if let Some(Token::LBrace) = self.tokens.get(self.position) {
                    self.position += 1;
                }
                if let Some(Token::Selector) = self.tokens.get(self.position) {
                    self.position += 1;
                }
                let selector = match self.tokens.get(self.position)? {
                    Token::String(s) => {
                        let sel = s.clone();
                        self.position += 1;
                        sel
                    }
                    _ => "".to_string(),
                };
                if let Some(Token::RBrace) = self.tokens.get(self.position) {
                    self.position += 1;
                }

                Some(Command::Scrape { url, selector })
            }
            _ => None,
        }
    }
}
