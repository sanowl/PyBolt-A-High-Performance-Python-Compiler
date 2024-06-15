pub mod lexer;

#[cfg(test)]
mod tests {
    use super::lexer::{Lexer, Token, TokenType};

    #[test]
    fn test_lex_identifier_or_keyword() {
        let mut lexer = Lexer::new("let x = 42;");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Keyword,
                lexeme: "let".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Identifier,
                lexeme: "x".to_string()
            }
        );
    }

    #[test]
    fn test_lex_number() {
        let mut lexer = Lexer::new("42");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Literal,
                lexeme: "42".to_string()
            }
        );
    }

    #[test]
    fn test_lex_operator() {
        let mut lexer = Lexer::new("+ - * / = ;");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Operator,
                lexeme: "+".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Operator,
                lexeme: "-".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Operator,
                lexeme: "*".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Operator,
                lexeme: "/".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Operator,
                lexeme: "=".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Punctuation,
                lexeme: ";".to_string()
            }
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("   let    x   = 42   ;   ");
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Keyword,
                lexeme: "let".to_string()
            }
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: TokenType::Identifier,
                lexeme: "x".to_string()
            }
        );
    }
}
