use super::token::{Token, TokenType};
macro_rules! is_whitespace {
    ($char:expr) => {
        matches!($char, ' ' | '\t' | '\n' | '\r')
    };
}

macro_rules! is_alphanumeric {
    ($char:expr) => {
        matches!($char, 'A'..='Z' | 'a'..='z')
    };
}

macro_rules! is_operator {
    ($char:expr) => {
        matches!($char, '+' | '-' | '/' | '*' | ';' | '(' | ')')
    };
}

macro_rules! is_numeric {
    ($char:expr) => {
        matches!($char, '0'..='9')
    };
}

pub struct Lexer {
    file_data: Vec<char>,
    current_index: usize,
    token_list: Vec<Token>,
}

impl Lexer {
    pub fn new(file_input: &String) -> Self {
        Self {
            file_data: file_input.chars().collect(),
            current_index: 0,
            token_list: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) {
        println!("The file length is {}\n", self.file_data.len());
        while self.current_index < self.file_data.len() {
            let current_byte = self.get_current_value();

            match current_byte {

                Some(byte) if *byte == ';' => {
                    self.add_token(Token::new(TokenType::TSemiColon, Some(0)));
                    break
                }


                Some(byte) if is_alphanumeric!(byte) => {
                    let mut current_word = String::new();

                    while let Some(next_byte) = self.get_current_value() {
                        if is_alphanumeric!(next_byte) {
                            current_word.push(*next_byte);
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    println!("The single character found is {}", current_word);
                    continue;
                },

                Some(byte) if is_operator!(byte) => {
                    println!("The single character found is {}", byte);
                    self.handle_operator(&byte.to_string());
                    self.consume();
                    continue;
                },

                Some(byte) if is_numeric!(byte) => {
                    let mut current_word = String::new();
                    
                    while let Some(next_byte) = self.get_current_value() {
                        if is_numeric!(next_byte) {
                            current_word.push(*next_byte);
                            self.consume(); // Consume only if it's a number
                        } else {
                            break;
                        }
                    }
                    println!("The single number found is {}", current_word);
                    self.add_token(Token::new(TokenType::TIntlit, Some(current_word.parse::<i64>().unwrap())));
                    continue;
                }

                Some(byte) if is_whitespace!(byte) => {
                    self.skip_whitespace();
                    continue;
                }

               
                Some(_) => {
                    self.consume(); // Consume unknown characters
                    continue;
                }

                None => break,
            }
        }
    }
}

impl Lexer {

    fn add_token(&mut self, cur_token: Token) {
        self.token_list.push(cur_token);
        return;
    }
    // Get the value at current_index
    fn get_current_value(&self) -> Option<&char> {
        self.file_data.get(self.current_index)
    }
    // Get the value at current_index + 1
    // Set current_index += 1
    fn consume(&mut self) -> Option<&char> {
        if self.current_index >= self.file_data.len() {
            return None;
        }
        let value: Option<&char> = self.file_data.get(self.current_index);
        self.current_index += 1;
        value
    }

    fn skip_whitespace(&mut self) {
        while let Some(byte) = self.get_current_value() {
            if is_whitespace!(*byte) {
                self.current_index += 1;
            } else {
                break;
            }
        }
    }
}

impl Lexer {
    fn handle_operator(&mut self, alpn_string: &str) {
        match alpn_string {
            "+" => {
                let cur_token = Token::new(TokenType::TAdd, None);
                self.add_token(cur_token);
            },
            "-" => {
                let cur_token = Token::new(TokenType::TSub, None);
                self.add_token(cur_token);
            },
            "*" => {
                let cur_token = Token::new(TokenType::TStar, None);
                self.add_token(cur_token);
            },
            "/" => {
                let cur_token = Token::new(TokenType::TSlash, None);
                self.add_token(cur_token);
            },
            "(" => {
                let cur_token = Token::new(TokenType::TLeftBracket, None);
                self.add_token(cur_token);
            },
            ")" => {
                let cur_token = Token::new(TokenType::TRightBracket, None);
                self.add_token(cur_token);
            }
            _ => {}
        }
        return;
    }
}


impl Lexer {

    pub fn print_tokens(&self) {
        println!("The number of tokens is {}\n", self.token_list.len());
        for token in self.token_list.iter() {
            println!("{:?}\n", token);
        }
        return;
    }
    pub fn get_tokens(&self) -> &Vec<Token> {
        return &self.token_list;
    }
}