use crate::chunk::Chunk;
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::token::{TokenType, Token};
use crate::opcode::OpCode;
pub struct Compiler<'a> { 
    scanner: Scanner,
    parser: Parser,
    chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn from_source(source: String, chunk: &'a mut Chunk) -> Self {
        Self {
            scanner: Scanner::new(&source),
            parser: Parser::new(),
            chunk: chunk,
        }
    }

    pub fn compile(&mut self) -> bool {
        
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expected end of expression");
        
        return !self.parser.hadError
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            // break when you encounter error
            if let TokenType::ERROR = self.parser.current.token_type {
                break
            }

            self.error_at_current();
        }
    }

    fn expression(&mut self ){
        todo!();
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
   
        if self.parser.current.token_type == token_type {
            self.advance();
            return
        }
        
        self.parser.current.message = Some(message.to_string());
        self.error_at_current()
    }
    
    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte, self.parser.previous.line);
    }
    fn emit_two_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compile(&mut self){
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8);
    }

    fn error_at_current(&mut self) {
        if self.parser.panicMode {
            return
        }
        self.parser.panicMode = true;
    
        let mut string = format!("{}: Error ", self.parser.current.line);
        match self.parser.current.token_type {
            TokenType::EOF => {
                string.push_str(" at end");
            },
            TokenType::ERROR => {
                string.push_str(" at end");
            }
            _ => {
                
                string.push_str(&format!("at {}", self.parser.current.start));
            }
        }

        if let Some(err_msg) = &self.parser.current.message {
            string.push_str(&format!("\n {}", err_msg));
        }

        eprintln!("{}", string);

        self.parser.hadError = true;
    }
}