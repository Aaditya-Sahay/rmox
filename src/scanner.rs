#![allow(dead_code)]

use crate::token::{Token, TokenType};

/// Our scanner, it reads through the source file and converts into tokens
pub struct Scanner {
    start: *const u8,
    current: *const u8,
    line: u64,
}

impl Scanner {
    /// creates a new instance of a scanner
    fn new(source: String) -> Self {
        Self {
            start: source.as_ptr(),
            current: source.as_ptr(),
            line: 0,
        }
    }
    /// scans a token
    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_end() {
            self.generate_token(TokenType::EOF);
        }

        let c = self.advance();

        match c {
            '(' => return self.generate_token(TokenType::LEFT_PAREN),
            ')' => return self.generate_token(TokenType::RIGHT_PAREN),
            '{' => return self.generate_token(TokenType::LEFT_BRACE),
            '}' => return self.generate_token(TokenType::RIGHT_BRACE),
            ';' => return self.generate_token(TokenType::SEMICOLON),
            ',' => return self.generate_token(TokenType::COMMA),
            '.' => return self.generate_token(TokenType::DOT),
            '-' => return self.generate_token(TokenType::MINUS),
            '+' => return self.generate_token(TokenType::PLUS),
            '/' => return self.generate_token(TokenType::SLASH),
            '*' => return self.generate_token(TokenType::STAR),
            '%' => return self.generate_token(TokenType::MOD),
            '!' => {
                let token = if self.match_char('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                return self.generate_token(token);
            }
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                return self.generate_token(token);
            }
            '<' => {
                let token = if self.match_char('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                return self.generate_token(token);
            }
            '>' => {
                let token = if self.match_char('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                return self.generate_token(token);
            }
            _ => return self.error_token("Unexpected character"),
        }
    }
    /// checks if scanner.current is at end.
    fn is_end(&self) -> bool {
        unsafe {
            return *self.current == '\0' as u8;
        }
    }
    fn peek(&self) -> char {
        unsafe { *self.current as char }
    }

    fn peek_next(&self) -> char {
        if self.is_end() {
            return '\0';
        }
        unsafe { *self.current.offset(1) as char }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        unsafe {
            if *self.current as char != expected {
                return false;
            }
            self.current = self.current.offset(1);

            return true;
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                }
                '/' => {
                    if self.peek_next() == '/'{
                        while self.peek() != '\n' && !self.is_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                    break;
                }
                _ => {}
            }
        }
    }

    /// it generates a token from a scan.
    fn generate_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: (self.current as usize - self.start as usize),
            line: self.line,
        }
    }
    /// it advances scanner.current by 1
    fn advance(&mut self) -> char {
        unsafe {
            self.current = self.current.offset(1);
            return *self.current.offset(-1) as char;
        }
    }
    /// it generates an error token
    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::ERROR,
            start: message.as_ptr(),
            length: message.len(),
            line: self.line,
        }
    }
}
