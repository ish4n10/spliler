pub enum TokenType {
    TAdd,
    TSub,
    TStar,
    TSlash,
    TIntlit,
}

pub struct Token {
    token: TokenType,
    value: i32,
}
