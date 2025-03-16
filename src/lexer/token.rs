

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    TAdd,
    TSub,
    TStar,
    TSlash,
    TIntlit,
}

#[derive(Copy, Clone, Debug)]
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
    pub fn get_token_type(&self) -> TokenType {
        self.token
    }
    pub fn get_token_value(&self) -> i64 {
        self.value
    }
}
