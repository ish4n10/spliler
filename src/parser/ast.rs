
use std::fmt;

#[derive(Clone, Debug)]
pub enum ASTNodeType {
    AAdd,
    ASub,
    AMult,
    ADivide,
    AIntLit,
    ALeftBracket,
    ARightBracket
}

#[derive(Clone, Debug)]
pub struct ASTNode {
    operation: ASTNodeType,
    left: Option<Box<ASTNode>>,
    right: Option<Box<ASTNode>>,
    value: Option<i64>,
}

impl ASTNode {
    pub fn new(operation: ASTNodeType, left: Option<Box<ASTNode>>, right: Option<Box<ASTNode>>, value: Option<i64>) -> Box<Self> {
        Box::new(ASTNode {
            operation,
            left,
            right,
            value
        })    
    }

    pub fn new_leaf(operation: ASTNodeType, value: i64) -> Box<Self> {
        Box::new(ASTNode {
            operation,
            left: None,
            right: None,
            value: Some(value)
        })
    }

    pub fn new_unary(operation: ASTNodeType, left: Option<Box<ASTNode>>, value: Option<i64>) -> Box<Self> {
        Box::new(ASTNode {
            operation,
            left,
            right: None,
            value
        })
    }
    
}


impl ASTNode {

    // this shit was not made by me 
    fn pretty_print_json(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth); 
        let node_type = format!("{:?}", self.operation); 

        writeln!(f, "{}{{", indent)?;
        writeln!(f, "{}  \"type\": \"{}\",", indent, node_type)?;

        if let Some(ref value) = self.value {
            writeln!(f, "{}  \"value\": \"{}\",", indent, value)?;
        }

        if let Some(ref left) = self.left {
            writeln!(f, "{}  \"left\": ", indent)?;
            left.pretty_print_json(f, depth + 1)?;
        }

        if let Some(ref right) = self.right {
            writeln!(f, "{}  \"right\": ", indent)?;
            right.pretty_print_json(f, depth + 1)?;
        }

        writeln!(f, "{}}}", indent)?;

        Ok(())
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_print_json(f, 0)
    }
}

