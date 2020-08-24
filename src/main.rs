pub mod chunk;
pub mod disassembler;
pub mod opcode;
pub mod scanner;
pub mod token;
pub mod vm;
pub mod compiler;
pub mod parser;
// use crate::chunk::Chunk;

// use crate::opcode::OpCode;
// use crate::vm::VM;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::vm::{VM,InterpretResult};
use crate::chunk::Chunk;
use  crate::compiler::Compiler;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    contents.push('\0');
    // let mut scanner = Scanner::new(&contents);
    // loop {
    //     let token = scanner.scan_token();
    //     println!("{:?}", token);
        
    //     if let TokenType::EOF = token.token_type {
    //         break;
    //     }
    // }

    interpret(contents);

    Ok(())
}



pub fn interpret(source: String) -> InterpretResult {
    let mut chunk = Chunk::new();
    let mut compiler = Compiler::from_source(source, &mut chunk);
    let compiled = compiler.compile();
    if !compiled {
        return InterpretResult::InterpretCompileError
    }


    let mut vm = VM::new(chunk);

    vm.interpret()
}