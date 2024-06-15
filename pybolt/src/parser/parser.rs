use crate::lexer::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    VariableDeclaration { name: String, value: Box<ASTNode> },
    Expression(ASTNodeType),
    Number(i32),
    BinaryOperation { left: Box<ASTNode>, operator: String, right: Box<ASTNode> },
}

#[derive(Debug)]
pub enum ASTNodeType {
    Identifier(String),
    Number(i32),
    BinaryOperation { operator: String, left: Box<ASTNode>, right: Box<ASTNode> },
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut nodes = Vec::new();
        while self.current_token.is_some() {
            nodes.push(self.parse_statement()?);
        }
        Ok(ASTNode::Program(nodes))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if self.match_token(TokenType::Keyword, "let") {
            self.parse_variable_declaration()
        } else {
            self.parse_expression()
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        self.expect_token(TokenType::Keyword, "let")?;
        let name = self.expect_token(TokenType::Identifier, "")?.lexeme.clone();
        self.expect_token(TokenType::Operator, "=")?;
        let value = self.parse_expression()?;
        self.expect_token(TokenType::Punctuation, ";")?;
        Ok(ASTNode::VariableDeclaration { name, value: Box::new(value) })
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_binary_operation(0)
    }

    fn parse_binary_operation(&mut self, min_precedence: u8) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.current_token.clone() {
            if let TokenType::Operator = op.token_type {
                let precedence = self.get_precedence(&op.lexeme);
                if precedence >= min_precedence {
                    self.current_token = self.lexer.next_token();
                    let mut right = self.parse_primary()?;
                    while let Some(next_op) = self.current_token.clone() {
                        if let TokenType::Operator = next_op.token_type {
                            let next_precedence = self.get_precedence(&next_op.lexeme);
                            if next_precedence > precedence {
                                right = self.parse_binary_operation(next_precedence)?;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    left = ASTNode::BinaryOperation { 
                        left: Box::new(left), 
                        operator: op.lexeme, 
                        right: Box::new(right) 
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let token = self.current_token.clone().ok_or("Unexpected end of input")?;
        self.current_token = self.lexer.next_token();

        match token.token_type {
            TokenType::Identifier => Ok(ASTNode::Expression(ASTNodeType::Identifier(token.lexeme))),
            TokenType::Literal => {
                let value = token.lexeme.parse::<i32>().map_err(|_| "Invalid number".to_string())?;
                Ok(ASTNode::Number(value))
            }
            _ => Err("Unexpected token".to_string())
        }
    }

    fn match_token(&self, token_type: TokenType, lexeme: &str) -> bool {
        self.current_token.as_ref().map_or(false, |token| {
            token.token_type == token_type && (lexeme.is_empty() || token.lexeme == lexeme)
        })
    }

    fn expect_token(&mut self, token_type: TokenType, lexeme: &str) -> Result<Token, String> {
        if self.match_token(token_type, lexeme) {
            let token = self.current_token.clone().unwrap();
            self.current_token = self.lexer.next_token();
            Ok(token)
        } else {
            Err(format!("Expected {:?} '{}'", token_type, lexeme))
        }
    }

    fn get_precedence(&self, operator: &str) -> u8 {
        match operator {
            "=" => 1,
            "+" | "-" => 2,
            "*" | "/" => 3,
            _ => 0,
        }
    }
}
