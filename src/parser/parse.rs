use std::thread::current;

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
    fn new(token_list: Vec<Token>) -> Self {
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
    // fn generate_ast(&mut self) -> Option<Box<ASTNode>> {
    //     self.assign_primary_node();
    //     if self.current.1.get_token_type() == TokenType::TSlash {
    //         return None;
    //     }
        
    //     let left = self.parse_tree_root.as_ref().unwrap();
    
    // }
    // maybe just maybe use stack instead of fucking recursion cuz fuck rust anyways
    // 2 + 3 * 5 + 7 -> [2, +, (3, *, 5) transform subtree] etc 
    // transform right heavy subtree -> something good tree idk lol 

    fn assign_primary_node(&mut self) {
        if let Some(token) = self.token_list.get(self.current.0) {
            if token.get_token_type() == TokenType::TIntlit {
                let node = ASTNode::new_leaf(ASTNodeType::AIntLit, token.get_token_value());
                self.parse_tree_root = Some(node);
                self.inc_current();
            }
        }
    }
}

impl Parser {

    fn inc_current(&mut self) {
        let next = self
            .token_list
            .get(self.current.0 + 1)
            .expect("Could not get the next token");
        self.current = (self.current.0 + 1, next.clone());
        return;
    }
}
