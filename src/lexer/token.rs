use std::collections::btree_map::VacantEntry;

#[derive(Debug)]
pub enum TokenType {
    TAdd,
    TSub,
    TStar,
    TSlash,
    TIntlit,
}

enum TokenValue {
   CharArray(String),
   Int(i64),
   Float(f64)
}

#[derive(Debug)]
pub struct Token {
    token: TokenType,
    value: i64,
}

impl Token {
    pub fn new(token: TokenType, value: Option<i64>) -> Self {
        if let Some(data) = value {
            return Token{token, value: data};
        }
        return Token{token, value: 0};
    }
}
