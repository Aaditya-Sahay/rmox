#![allow(dead_code)]

use crate::token::{Token, TokenType};

/// Our scanner, it reads through the source file and converts into tokens
pub struct Scanner {
    start: usize,
    current: usize,
    source_vec: Vec<char>,
    line: u64,
}

impl Scanner {
    /// creates a new instance of a scanner
    pub fn new(source: &str) -> Self {
        Self {
            start: 0,
            current: 0,
            source_vec: source.chars().collect(),
            line: 0,
        }
    }
    /// scans a token
    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        
        self.skip_whitespace();

        if self.is_end() {
            return self.generate_token(TokenType::EOF)
        }


        let c = self.advance();

        match c {
            '(' => self.generate_token(TokenType::LEFT_PAREN),
            ')' => self.generate_token(TokenType::RIGHT_PAREN),
            '{' => self.generate_token(TokenType::LEFT_BRACE),
            '}' => self.generate_token(TokenType::RIGHT_BRACE),
            ';' => self.generate_token(TokenType::SEMICOLON),
            ',' => self.generate_token(TokenType::COMMA),
            '.' => self.generate_token(TokenType::DOT),
            '-' => self.generate_token(TokenType::MINUS),
            '+' => self.generate_token(TokenType::PLUS),
            '/' => self.generate_token(TokenType::SLASH),
            '*' => self.generate_token(TokenType::STAR),
            '%' => self.generate_token(TokenType::MOD),
            '!' => {
                let token = if self.match_char('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.generate_token(token)
            }
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.generate_token(token)
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
                self.generate_token(token)
            }
            '"' => self.handle_string(),
            '0'..='9' => self.handle_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(),
            _ => self.error_token("Unexpected character"),
        }
    }
    /// checks if scanner.current is at end.
    fn is_end(&self) -> bool {
        if self.current >= self.source_vec.len() - 1 {
            return true
        }
        return self.source_vec[self.current] == '\0';
    }
    fn peek(&self) -> char {
        self.source_vec[self.current]
    }

    fn peek_next(&self) -> char {
        if self.is_end() {
            return '\0';
        }
        self.source_vec[self.current + 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

            if self.source_vec[self.current] != expected {
                return false;
            }

            self.current += 1;
            return true;
   
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_end() {break;}
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
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                    break;
                }
                _ => break,
            }
        }
    }

    fn handle_string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return self.error_token("Unterminated string");
        }

        self.advance();
        self.generate_token(TokenType::STRING)
    }

    fn handle_number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.generate_token(TokenType::NUMBER)
    }

    fn handle_identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let token_type = self.get_identifier_type();
        self.generate_token(token_type)
    }

    fn get_identifier_type(&mut self) -> TokenType {
        let c;
        c = self.source_vec[self.start];

        match c {
            'a' => self.check_keyword(1, 2, "nd", TokenType::AND),
            'c' => self.check_keyword(1, 4, "lass", TokenType::CLASS),
            'e' => self.check_keyword(1, 3, "lse", TokenType::ELSE),
            'i' => self.check_keyword(1, 1, "f", TokenType::IF),
            'n' => self.check_keyword(1, 2, "il", TokenType::NIL),
            'o' => self.check_keyword(1, 1, "r", TokenType::OR),
            'p' => self.check_keyword(1, 4, "rint", TokenType::PRINT),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::RETURN),
            's' => self.check_keyword(1, 4, "uper", TokenType::SUPER),
            'd' => self.check_keyword(1, 2, "ec", TokenType::VAR),
            'w' => self.check_keyword(1, 4, "hile", TokenType::WHILE),
            'f' => {
                if self.current as u8 - self.start as u8 > 1 {
                    match self.source_vec[self.current + 1] {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::FALSE),
                        'o' => self.check_keyword(2, 1, "r", TokenType::FOR),
                        'u' => self.check_keyword(2, 2, "nc", TokenType::FUN),
                        _ => return TokenType::IDENTIFIER,
                    }
                } else {
                    return TokenType::IDENTIFIER;
                }
            }
            't' => {
                if self.current as u8 - self.start as u8 > 1 {
                    match  self.source_vec[self.current + 1]  {
                        'h' => self.check_keyword(2, 2, "is", TokenType::THIS),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::TRUE),
                        _ => return TokenType::IDENTIFIER,
                    }
                } else {
                    return TokenType::IDENTIFIER;
                }
            }

            _ => return TokenType::IDENTIFIER,
        }
    }

    fn check_keyword(
        &mut self,
        start: u8,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current as u8 - self.start as u8 == start + length as u8 {
            let mut correct = true;
            for (index, character) in rest.chars().rev().enumerate() {
                let i = self.current - index - 1;
                if self.source_vec[i] != character {
                    correct = false;
                    break;
                }
            }

            if correct {
                return token_type
            }
        }

        return TokenType::IDENTIFIER;
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
        self.current += 1;
        self.source_vec[self.current - 1]
    }
    /// it generates an error token
    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::ERROR,
            start: self.start,
            length: message.len(),
            line: self.line,
        }
    }
}
