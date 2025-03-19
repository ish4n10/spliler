
use crate::helpers::get_ast_by_token_type;
use crate::lexer::token::Token;
use crate::lexer::token::TokenType;

use super::ast::{ASTNode, ASTNodeType};

pub struct Parser {
    current: (usize, Token),
    token_list: Vec<Token>,
    parse_tree_root: Option<Box<ASTNode>>,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        if token_list.is_empty() {
            panic!("Please specify token_list for parser");
        }
        let first_value = token_list
            .get(0)
            .expect("Could not get the first token in Parser::new");
        Parser {
            current: (0, first_value.clone()),
            token_list,
            parse_tree_root: None,
        }
    }
}

impl Parser {
    pub fn generate_ast(&mut self) -> Option<Box<ASTNode>> {
        let mut value_stack: Vec<Box<ASTNode>> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();
        
        self.assign_primary_node();
        value_stack.push(self.parse_tree_root.take()?);

        while let Some(token) = self.token_list.get(self.current.0).cloned() {
            if token.get_token_type() == TokenType::TSemiColon  {
                break;
            }

            self.inc_current();
            self.assign_primary_node();

            let right_value: Box<ASTNode> = self.parse_tree_root.take()?;

            let left_value: Box<ASTNode> = value_stack.pop().unwrap(); // the previous value
            let new_node: Box<ASTNode> = ASTNode::new(get_ast_by_token_type(token.get_token_type()), Some(left_value), Some(right_value), Some(token.get_token_value()));
            value_stack.push(new_node);
        }
        value_stack.pop()
}
    
    
    // maybe just maybe use stack instead of fucking recursion cuz fuck rust anyways
    // 2 + 3 * 5 + 7 -> [2, +, (3, *, 5) transform subtree] etc 
    // transform right heavy subtree -> something good tree idk lol 

    fn assign_primary_node(&mut self) {
        if let Some(token) = self.token_list.get(self.current.0) {
            if token.get_token_type() == TokenType::TIntlit {
                let node: Box<ASTNode> = ASTNode::new_leaf(ASTNodeType::AIntLit, token.get_token_value());
                self.parse_tree_root = Some(node);
                self.inc_current();
            }
        }
    }
}

impl Parser {

    fn inc_current(&mut self) {
        if self.current.0 + 1 >= self.token_list.len() {
            return; 
        }
        self.current.0 += 1;
        self.current.1 = self.token_list[self.current.0].clone();
    }
}
