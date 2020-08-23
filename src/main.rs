// since nesse is here I am going to give it another try.

/*
    name: Take 1
*/

pub mod chunk;
pub mod disassembler;
pub mod opcode;
pub mod scanner;
pub mod token;
pub mod vm;

// use crate::chunk::Chunk;

// use crate::opcode::OpCode;
// use crate::vm::VM;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::scanner::Scanner;
use crate::token:: {TokenType};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    contents.push('\0');
    let mut scanner = Scanner::new(&contents);
    loop {
        let token = scanner.scan_token();
        println!("{:?}", token);
        
        if let TokenType::EOF = token.token_type {
            break;
        }
    }

    Ok(())
}
