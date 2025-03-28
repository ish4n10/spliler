mod helpers;
mod lexer;
mod parser;
mod codegen;

use helpers::read_input_file;
use parser::parse::Parser;
use std::env;
use codegen::cpu::instructions;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a file name!");
        return;
    }
    let file_data = read_input_file(&args[1]).unwrap();
    let mut lexer = lexer::lexical_analysis::Lexer::new(&file_data);

    lexer.tokenize();

    lexer.print_tokens();
    let mut parser: Parser = Parser::new(lexer.get_tokens().to_vec());
    let something = parser.additive_expr();
    println!("The data is \n{}", something.unwrap());
    println!("The file data is\n{}", file_data);
    
    return;
}
