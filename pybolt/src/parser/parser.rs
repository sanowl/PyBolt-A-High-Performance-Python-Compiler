use crate::lexer::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        while let Some(token) = &self.current_token {
            println!("Parsed token: {:?}", token);
            self.current_token = self.lexer.next_token();
        }
        Ok(())
    }
}
