#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Literal,
    Punctuation,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token.token_type != TokenType::EOF {
                tokens.push(token);
            } else {
                break;
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Some(Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
            });
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        if current_char.is_alphabetic() {
            return Some(self.lex_identifier_or_keyword());
        } else if current_char.is_digit(10) {
            return Some(self.lex_number());
        } else {
            match current_char {
                '=' => self.create_token(TokenType::Operator),
                ';' => self.create_token(TokenType::Punctuation),
                '+' | '-' | '*' | '/' => self.create_token(TokenType::Operator),
                _ => None,
            }
        }
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start_pos = self.position;
        while let Some(next_char) = self.input.chars().nth(self.position) {
            if next_char.is_alphanumeric() || next_char == '_' {
                self.position += 1;
            } else {
                break;
            }
        }
        let lexeme = &self.input[start_pos..self.position];
        let token_type = if lexeme == "let" || lexeme == "fn" {
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
        let start_pos = self.position;
        while let Some(next_char) = self.input.chars().nth(self.position) {
            if next_char.is_digit(10) {
                self.position += 1;
            } else {
                break;
            }
        }
        Token {
            token_type: TokenType::Literal,
            lexeme: self.input[start_pos..self.position].to_string(),
        }
    }

    fn create_token(&mut self, token_type: TokenType) -> Option<Token> {
        let lexeme = self.input.chars().nth(self.position).unwrap().to_string();
        self.position += 1;
        Some(Token { token_type, lexeme })
    }

    fn skip_whitespace(&mut self) {
        while let Some(next_char) = self.input.chars().nth(self.position) {
            if next_char.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
}
