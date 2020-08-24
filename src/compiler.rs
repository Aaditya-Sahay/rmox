use crate::chunk::Chunk;
use crate::scanner::Scanner;
use crate::parser::*;
use crate::token::{TokenType};
use crate::opcode::OpCode;
use crate::disassembler::Disassembler;

pub struct Compiler<'a> { 
    scanner: Scanner,
    parser: Parser,
    chunk: &'a mut Chunk,
    source: Vec<char>
}

impl<'a> Compiler<'a> {
    pub fn from_source(source: String, chunk: &'a mut Chunk) -> Self {
        Self {
            // change scanner to take vector of characters 
            scanner: Scanner::new(&source),
            parser: Parser::new(),
            chunk: chunk,
            source: source.chars().collect()
        }
    }

    pub fn compile(&mut self) -> bool {
        
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expected end of expression");
        self.end_compile();
        
        return !self.parser.had_error
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            // break when you dont encounter error
            // println!("{:?}", self.parser.previous);

            match self.parser.current.token_type {
                TokenType::ERROR => {},
                _ => { break; }
            }
            // if let TokenType::ERROR = self.parser.current.token_type {
            //     break
            // }

            self.error_at_current();
      
        }
    }

    fn expression(&mut self ){
      
        self.parse_precedence(Precedence::ASSIGNMENT);
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

        Disassembler::disassemble_chunk(self.chunk, "Compiled Chunk".to_string());
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8);
    }
    fn emit_constant(&mut self, value: f64) {
        let byte2 = self.make_constant(value);
        self.emit_two_bytes(OpCode::OpConstant as u8, byte2)
    }
    fn make_constant(&mut self, value: f64) -> u8{
        let constant = self.chunk.add_constant(value);
        if constant > std::u8::MAX {
            eprintln!("Too many constants in one chunk");
            return 0
        }

        return constant
    }

    fn handle_number(&mut self ){
        let mut string: String = String::new();
      
        for i in self.parser.previous.start..self.parser.previous.start + self.parser.previous.length {
            string.push(self.source[i]);
        }
 
        let value:f64 = string.trim().parse().unwrap();
        self.emit_constant(value);
    }

    fn handle_grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression");
    }

    fn handle_unary(&mut self) {
        let token_type = self.parser.previous.token_type;
        self.parse_precedence(Precedence::UNARY);
      
        match token_type {
            TokenType::MINUS => self.emit_byte(OpCode::OpNegate as u8),
            _ => {return}
        }
    }

    fn handle_binary(&mut self) {
        
        let operator_type = self.parser.previous.token_type;
        let rule = self.get_rule(operator_type);

        let precedence = unsafe { std::mem::transmute(rule.precedence as u8 + 1 as u8) };
       
        self.parse_precedence(precedence);
        
        match operator_type {
            TokenType::PLUS => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::MINUS => self.emit_byte(OpCode::OpSub as u8),
            TokenType::STAR  => self.emit_byte(OpCode::OpMult as u8),
            TokenType::SLASH => self.emit_byte(OpCode::OpDiv as u8),
            TokenType::MOD => self.emit_byte(OpCode::OpMod as u8),
            _ => {return}
        }
    }
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
   
        let token_type = self.parser.previous.token_type;
        let rule = self.get_rule(token_type);
       

        match rule.prefix {
            ParseFunctions::Binary => self.handle_binary(),
            ParseFunctions::Unary =>  { self.handle_unary() },
            ParseFunctions::Number => self.handle_number(),
            ParseFunctions::Grouping => self.handle_grouping(),
            _ => { return }
        }

        let current_token_type = self.parser.current.token_type;

        while precedence as u8 <= self.get_rule(current_token_type).precedence as u8 {
            self.advance();
            let prev_token_type = self.parser.previous.token_type;
            let infix_rule = self.get_rule(prev_token_type);
            match infix_rule.infix {
                ParseFunctions::Binary => self.handle_binary(),
                ParseFunctions::Unary => self.handle_unary(),
                ParseFunctions::Number => self.handle_number(),
                ParseFunctions::Grouping => self.handle_grouping(),
                _ => { return }
            }
        }  
    }

    fn get_rule(&self, token_type: TokenType) ->ParseRule{
        self.parser.parse_rules[token_type as usize]
    }

    fn error_at_current(&mut self) {
        if self.parser.panic_mode {
            return
        }
        self.parser.panic_mode = true;
    
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

        self.parser.had_error = true;
    }
}