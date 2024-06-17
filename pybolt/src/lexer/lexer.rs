#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Literal,
    Punctuation,
    Eof,
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
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Some(Token { token_type: TokenType::Eof, lexeme: "".to_string() });
        }

        let current_char = self.input.as_bytes()[self.position] as char;

        let token = if current_char.is_alphabetic() {
            self.identifier()
        } else if current_char.is_digit(10) {
            self.number()
        } else {
            self.operator_or_punctuation()
        };

        self.position += token.lexeme.len();
        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input.as_bytes()[self.position].is_ascii_whitespace() {
            self.position += 1;
        }
    }

    fn identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input.as_bytes()[self.position].is_ascii_alphabetic() {
            self.position += 1;
        }
        let lexeme = &self.input[start..self.position];
        let token_type = if lexeme == "let" { TokenType::Keyword } else { TokenType::Identifier };
        Token { token_type, lexeme: lexeme.to_string() }
    }

    fn number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input.as_bytes()[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let lexeme = &self.input[start..self.position];
        Token { token_type: TokenType::Literal, lexeme: lexeme.to_string() }
    }

    fn operator_or_punctuation(&mut self) -> Token {
        let current_char = self.input.as_bytes()[self.position] as char;
        let token_type = match current_char {
            '=' | '+' | '-' | '*' | '/' => TokenType::Operator,
            ';' => TokenType::Punctuation,
            _ => panic!("Unexpected character: {}", current_char),
        };
        Token { token_type, lexeme: current_char.to_string() }
    }
}
