use super::token::Token;


macro_rules! is_whitespace {
    ($char:expr) => {
        matches!($char, ' ' | '\t' | '\n' | '\r')
    };
}

macro_rules! is_alphanumeric {
    ($char:expr) => {
        matches!($char, '0'..='9' | 'A'..='Z' | 'a'..='z')
    };
}

macro_rules! is_numeric {
    ($char:expr) => {
        matches!($char, '0'..='9')
    };
}

pub struct Parser {
    file_data: Vec<char>,
    current_index: usize,
    token_list: Vec<Token>,
}

impl Parser {
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

impl Parser {

    // Get the value at current_index
    fn get_current_value(&self) -> Option<&char> {
        self.file_data.get(self.current_index)
    }

     // Get the value at current_index + 1
    fn get_next_value(&self) -> Option<&char> {
        self.file_data.get(self.current_index + 1)
    }

    // Get the value at current_index + 1
    // Set current_index += 1
    fn consume(&mut self) -> Option<&char> {
        if self.current_index >= self.file_data.len() {
            return None;
        }
        let value = self.file_data.get(self.current_index);
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