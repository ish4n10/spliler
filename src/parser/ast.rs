pub enum ASTNodeType {
    AAdd,
    ASub,
    AMult,
    ADivide,
    AIntLit
}

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

