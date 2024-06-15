#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: String,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }
        let current_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1;
        Some(Token {
            lexeme: current_char.to_string(),
        })
    }
}
