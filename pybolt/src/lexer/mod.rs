pub mod lexer;

#[cfg(test)]
mod tests {
    use super::lexer::{Lexer, Token, TokenType};

    fn assert_token(lexer: &mut Lexer, expected_type: TokenType, expected_lexeme: &str) {
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                token_type: expected_type,
                lexeme: expected_lexeme.to_string()
            }
        );
    }

    #[test]
    fn test_lex_identifier_or_keyword() {
        let mut lexer = Lexer::new("let x = 42;");
        assert_token(&mut lexer, TokenType::Keyword, "let");
        assert_token(&mut lexer, TokenType::Identifier, "x");
        assert_token(&mut lexer, TokenType::Operator, "=");
        assert_token(&mut lexer, TokenType::Literal, "42");
        assert_token(&mut lexer, TokenType::Punctuation, ";");
    }

    #[test]
    fn test_lex_number() {
        let mut lexer = Lexer::new("42 3.14 -5");
        assert_token(&mut lexer, TokenType::Literal, "42");
        assert_token(&mut lexer, TokenType::Literal, "3.14");
        assert_token(&mut lexer, TokenType::Operator, "-");
        assert_token(&mut lexer, TokenType::Literal, "5");
    }

    #[test]
    fn test_lex_operator() {
        let mut lexer = Lexer::new("+ - * / = == != < > <= >=");
        let operators = vec!["+", "-", "*", "/", "=", "==", "!=", "<", ">", "<=", ">="];
        for op in operators {
            assert_token(&mut lexer, TokenType::Operator, op);
        }
    }

    #[test]
    fn test_lex_punctuation() {
        let mut lexer = Lexer::new("( ) { } [ ] , ; .");
        let punctuation = vec!["(", ")", "{", "}", "[", "]", ",", ";", "."];
        for p in punctuation {
            assert_token(&mut lexer, TokenType::Punctuation, p);
        }
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("   let    x   = 42   ;   ");
        assert_token(&mut lexer, TokenType::Keyword, "let");
        assert_token(&mut lexer, TokenType::Identifier, "x");
        assert_token(&mut lexer, TokenType::Operator, "=");
        assert_token(&mut lexer, TokenType::Literal, "42");
        assert_token(&mut lexer, TokenType::Punctuation, ";");
    }

    #[test]
    fn test_lex_string_literal() {
        let mut lexer = Lexer::new(r#""Hello, world!" 'c'"#);
        assert_token(&mut lexer, TokenType::Literal, "\"Hello, world!\"");
        assert_token(&mut lexer, TokenType::Literal, "'c'");
    }

    #[test]
    fn test_lex_complex_expression() {
        let mut lexer = Lexer::new("if (x > 0 && y < 10) { result = x + y * 2; }");
        let expected_tokens = vec![
            (TokenType::Keyword, "if"),
            (TokenType::Punctuation, "("),
            (TokenType::Identifier, "x"),
            (TokenType::Operator, ">"),
            (TokenType::Literal, "0"),
            (TokenType::Operator, "&&"),
            (TokenType::Identifier, "y"),
            (TokenType::Operator, "<"),
            (TokenType::Literal, "10"),
            (TokenType::Punctuation, ")"),
            (TokenType::Punctuation, "{"),
            (TokenType::Identifier, "result"),
            (TokenType::Operator, "="),
            (TokenType::Identifier, "x"),
            (TokenType::Operator, "+"),
            (TokenType::Identifier, "y"),
            (TokenType::Operator, "*"),
            (TokenType::Literal, "2"),
            (TokenType::Punctuation, ";"),
            (TokenType::Punctuation, "}"),
        ];

        for (expected_type, expected_lexeme) in expected_tokens {
            assert_token(&mut lexer, expected_type, expected_lexeme);
        }
    }
}
