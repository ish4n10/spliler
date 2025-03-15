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
    fn new(operation: ASTNodeType, left: Option<Box<ASTNode>>, right: Option<Box<ASTNode>>, value: Option<i64>) -> Box<Self> {
        Box::new(ASTNode {
            operation,
            left,
            right,
            value
        })    
    }
}