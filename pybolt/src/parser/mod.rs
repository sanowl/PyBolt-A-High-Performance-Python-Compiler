pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::{Parser, ASTNode, ASTNodeType};
    use crate::lexer::lexer::Lexer;
    use crate::lexer::lexer::Token;

    #[test]
    fn test_variable_declaration() {
        let input = "let x = 42;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().expect("Parsing failed");

        if let ASTNode::Program(nodes) = ast {
            assert_eq!(nodes.len(), 1);
            if let ASTNode::VariableDeclaration { name, value } = &nodes[0] {
                assert_eq!(name, "x");
                if let ASTNode::Number(n) = **value {
                    assert_eq!(n, 42);
                } else {
                    panic!("Expected a number");
                }
            } else {
                panic!("Expected a variable declaration");
            }
        } else {
            panic!("Expected a program node");
        }
    }

    #[test]
    fn test_arithmetic_expression() {
        let input = "let x = 1 + 2 * 3;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().expect("Parsing failed");

        if let ASTNode::Program(nodes) = ast {
            assert_eq!(nodes.len(), 1);
            if let ASTNode::VariableDeclaration { name, value } = &nodes[0] {
                assert_eq!(name, "x");
                if let ASTNode::BinaryOperation { left, operator, right } = &**value {
                    assert_eq!(operator, "+");
                    if let ASTNode::Number(n) = **left {
                        assert_eq!(n, 1);
                    } else {
                        panic!("Expected a number");
                    }
                    if let ASTNode::BinaryOperation { left, operator, right } = &**right {
                        assert_eq!(operator, "*");
                        if let ASTNode::Number(n) = **left {
                            assert_eq!(n, 2);
                        } else {
                            panic!("Expected a number");
                        }
                        if let ASTNode::Number(n) = **right {
                            assert_eq!(n, 3);
                        } else {
                            panic!("Expected a number");
                        }
                    } else {
                        panic!("Expected a binary operation");
                    }
                } else {
                    panic!("Expected a binary operation");
                }
            } else {
                panic!("Expected a variable declaration");
            }
        } else {
            panic!("Expected a program node");
        }
    }

    #[test]
    fn test_invalid_syntax() {
        let input = "let x = ;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_statements() {
        let input = "let x = 42; let y = x + 5;";
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse().expect("Parsing failed");

        if let ASTNode::Program(nodes) = ast {
            assert_eq!(nodes.len(), 2);

            // Test first variable declaration
            if let ASTNode::VariableDeclaration { name, value } = &nodes[0] {
                assert_eq!(name, "x");
                if let ASTNode::Number(n) = **value {
                    assert_eq!(n, 42);
                } else {
                    panic!("Expected a number");
                }
            } else {
                panic!("Expected a variable declaration");
            }

            // Test second variable declaration
            if let ASTNode::VariableDeclaration { name, value } = &nodes[1] {
                assert_eq!(name, "y");
                if let ASTNode::BinaryOperation { left, operator, right } = &**value {
                    assert_eq!(operator, "+");
                    if let ASTNode::Expression(ASTNodeType::Identifier(id)) = **left {
                        assert_eq!(id, "x");
                    } else {
                        panic!("Expected an identifier");
                    }
                    if let ASTNode::Number(n) = **right {
                        assert_eq!(n, 5);
                    } else {
                        panic!("Expected a number");
                    }
                } else {
                    panic!("Expected a binary operation");
                }
            } else {
                panic!("Expected a variable declaration");
            }
        } else {
            panic!("Expected a program node");
        }
    }
}
