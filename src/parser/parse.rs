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
    fn generate_ast(&mut self) {}

    fn assign_primary_node(&mut self) {
        match self.current.1.get_token_type() {
            TokenType::TIntlit => {
                let node =
                    ASTNode::new_leaf(ASTNodeType::AIntLit, self.current.1.get_token_value());
                self.parse_tree_root = Some(node);
                self.inc_current();
            }
            _ => panic!("Syntax error"),
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
