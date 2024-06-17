#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    Literal,
    Operator,
    Punctuation,
    Eof
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
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
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return None;
        }
        let current_char = self.input.as_bytes()[self.position] as char;
        if current_char.is_alphabetic() {
            return Some(self.lex_identifier_or_keyword());
        }
        if current_char.is_digit(10) {
            return Some(self.lex_number());
        }
        if "+-*/=;".contains(current_char) {
            return Some(self.lex_operator_or_punctuation());
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input.as_bytes()[self.position].is_ascii_whitespace() {
            self.position += 1;
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input.as_bytes()[self.position].is_ascii_alphabetic() {
            self.position += 1;
        }
        let lexeme = &self.input[start..self.position];
        let token_type = if lexeme == "let" {
            TokenType::Keyword
        } else {
            TokenType::Identifier
        };
        Token {
            token_type,
            lexeme: lexeme.to_string(),
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input.as_bytes()[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let lexeme = &self.input[start..self.position];
        Token {
            token_type: TokenType::Literal,
            lexeme: lexeme.to_string(),
        }
    }

    fn lex_operator_or_punctuation(&mut self) -> Token {
        let current_char = self.input.as_bytes()[self.position] as char;
        self.position += 1;
        let token_type = if current_char == ';' {
            TokenType::Punctuation
        } else {
            TokenType::Operator
        };
        Token {
            token_type,
            lexeme: current_char.to_string(),
        }
    }
}
