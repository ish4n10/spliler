use crate::helpers::get_ast_by_token_type;
use crate::lexer::token::Token;
use crate::lexer::token::TokenType;

use super::ast::{ASTNode, ASTNodeType};

pub struct Parser {
    current: (usize, Token),
    token_list: Vec<Token>,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        if token_list.is_empty() {
            panic!("Please specify token_list for parser");
        }
        let first_value: &Token = token_list
            .get(0)
            .expect("Could not get the first token in Parser::new");
        Parser {
            current: (0, first_value.clone()),
            token_list,
        }
    }

    fn inc_current(&mut self) {
        if self.current.0 + 1 >= self.token_list.len() {
            return;
        }
        self.current.0 += 1;
        self.current.1 = self.token_list[self.current.0].clone();
    }
}

impl Parser {
    // Handles + and -
    pub fn additive_expr(&mut self) -> Option<Box<ASTNode>> {
        let mut left: Box<ASTNode> = self.multiplicative_expr()?;

        while let Some(token) = self.token_list.get(self.current.0).cloned() {
            match token.get_token_type() {
                TokenType::TAdd | TokenType::TSub => {
                    let op_type: ASTNodeType = get_ast_by_token_type(token.get_token_type());

                    self.inc_current();
                    let right: Box<ASTNode> = self.multiplicative_expr()?;

                    left = ASTNode::new(
                        op_type,
                        Some(left),
                        Some(right),
                        Some(token.get_token_value()),
                    );
                }
                _ => break,
            }
        }
        Some(left)
    }

    // Handles * and /
    fn multiplicative_expr(&mut self) -> Option<Box<ASTNode>> {
        let mut left: Box<ASTNode> = self.integer_expr()?;

        while let Some(token) = self.token_list.get(self.current.0).cloned() {
            match token.get_token_type() {
                TokenType::TStar | TokenType::TSlash => {
                    let op_type: ASTNodeType = get_ast_by_token_type(token.get_token_type());

                    self.inc_current();
                    let right: Box<ASTNode> = self.integer_expr()?;

                    left = ASTNode::new(
                        op_type,
                        Some(left),
                        Some(right),
                        Some(token.get_token_value()),
                    );
                }
                _ => break,
            }
        }
        Some(left)
    }

    // give an additive expr until the bracket close? might be something lol
    fn paranthesis_expr(&mut self) -> Option<Box<ASTNode>> {
       
       print!("YAY I AM CALLED");
       if let Some(token) = self.token_list.get(self.current.0).cloned() {
            if token.get_token_type() == TokenType::TLeftBracket {
                
                self.inc_current();
                let expr_node: Option<Box<ASTNode>> = self.additive_expr();

                if let Some(token) = self.token_list.get(self.current.0) {
                    if token.get_token_type() == TokenType::TRightBracket {
                        self.inc_current();
                        return expr_node;
                    }
                } else {
                    panic!("Syntax error: Expected ')' but found {:?}", token);
                }
            } else {
                panic!("Syntax error: Unexpected end lol");
            }
       }
       None
    }

    fn integer_expr(&mut self) -> Option<Box<ASTNode>> {
        if let Some(token) = self.token_list.get(self.current.0) {
            if token.get_token_type() == TokenType::TIntlit {
                let node: Box<ASTNode> =
                    ASTNode::new_leaf(ASTNodeType::AIntLit, token.get_token_value());
                self.inc_current();

                if let Some(token) = self.token_list.get(self.current.0) {
                    if token.get_token_type() == TokenType::TIntlit {
                        panic!("The syntax is invalid in {}", token.get_token_value())
                    }
                    
                }
                return Some(node);
            }
            else if token.get_token_type() == TokenType::TLeftBracket {
                return self.paranthesis_expr();
            }
        }
        None
    }
}


