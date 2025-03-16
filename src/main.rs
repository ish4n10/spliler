mod helpers;
mod lexer;
mod parser;

use helpers::read_input_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a file name!");
        return;
    }
    let file_data = read_input_file(&args[1]).unwrap();

    println!("The file data is\n{}", file_data);
    return;
}
