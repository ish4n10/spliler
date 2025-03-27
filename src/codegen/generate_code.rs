use crate::parser::ast::{ASTNodeType, ASTNode};

fn generate_code(root: &Box<ASTNode>)  {
    
    let left: Option<Box<ASTNode>> = root.get_left();
    let right: Option<Box<ASTNode>> = root.get_right();

    
    match root.get_operation() {
        ASTNodeType::AAdd => todo!(),
        ASTNodeType::ASub => todo!(),
        ASTNodeType::AMult => todo!(),
        ASTNodeType::ADivide => todo!(),
        ASTNodeType::AIntLit => todo!(),
        ASTNodeType::ALeftBracket => todo!(),
        ASTNodeType::ARightBracket => todo!(),

        _ => panic!("Invalid operation"),
    }
}