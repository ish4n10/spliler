use std::{fs::File, io::Read, path::Path};

use crate::parser::ast::ASTNodeType;
use crate::lexer::token::TokenType;

pub fn read_input_file(file_path: &str) -> Result<String, String> {
    let file_name = file_path.replace("/", "");
    if file_name.is_empty() {
        return Err("The file name is empty ".to_string());
    }

    let path = Path::new(&file_name);
    if !path.exists() {
        return Err("Path not found".to_string());
    }

    let mut file_content = Vec::new();

    let mut file = File::open(&file_name).expect("you fucked up");
    file.read_to_end(&mut file_content).expect("you fucked up");

    return Ok(String::from_utf8_lossy(&file_content).into());
}

pub fn get_ast_by_token_type(token: TokenType) -> ASTNodeType {
    match token {
        TokenType::TAdd => ASTNodeType::AAdd,
        TokenType::TSub => ASTNodeType::ASub,
        TokenType::TStar => ASTNodeType::AMult,
        TokenType::TSlash => ASTNodeType::ADivide,
        TokenType::TIntlit => ASTNodeType::AIntLit,
        TokenType::TLeftBracket => ASTNodeType::ALeftBracket,
        TokenType::TRightBracket => ASTNodeType::ARightBracket,
        _ => panic!("The token could not be found while matching ast instruction!")

    }
}
