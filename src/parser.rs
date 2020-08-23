use crate::token::{Token, TokenType};
pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub hadError: bool,
    pub panicMode: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token { token_type: TokenType::EOF, start: 0, length: 0, line: 0, message: None},
            previous: Token { token_type: TokenType::EOF, start: 0, length: 0, line: 0, message: None},
            hadError: false,
            panicMode: false
        }
    }
}