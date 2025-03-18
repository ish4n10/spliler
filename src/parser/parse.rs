
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
        self.assign_primary_node();
        let mut root = self.parse_tree_root.take()?;
    
        while self.current.0 < self.token_list.len() {

            // i have no idea how you fucking cannot borrow self as mutable in a fucking while-let 
            // imagine cloning just for the sake 
            // pls help
            let token = self.token_list[self.current.0].clone(); 
            if token.get_token_type() == TokenType::TSlash {
                break;
            }
    
            self.inc_current(); // this assumes there NOT be any syntax error in the token list 
            //  
            self.assign_primary_node();
            let right = self.parse_tree_root.take()?;
    
            root = ASTNode::new(
                get_ast_by_token_type(token.get_token_type()),
                Some(root),
                Some(right),
                Some(token.get_token_value()),
            );
        }
        Some(root)
    }
    
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
        if self.current.0 + 1 >= self.token_list.len() {
            return; 
        }
        self.current.0 += 1;
        self.current.1 = self.token_list[self.current.0].clone();
    }
}
