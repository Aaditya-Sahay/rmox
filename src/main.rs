// since nesse is here I am going to give it another try.


/*
    name: Take 1
*/


pub mod chunk;
pub mod disassembler;
pub mod opcode;
pub mod vm;
pub mod scanner;
pub mod token;

// use crate::chunk::Chunk;

// use crate::opcode::OpCode;
// use crate::vm::VM;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // let chunk = Chunk::new();
    // let mut vm = VM::new(chunk);
    // let constant: usize = vm.chunk.add_constant(20 as f64);
    // vm.chunk.write_chunk(OpCode::OpConstant as u8, 100);
    // vm.chunk.write_chunk(constant as u8, 100);
    // vm.chunk.write_chunk(OpCode::OpNegate as u8, 100);
    // vm.chunk.write_chunk(OpCode::OpReturn as u8, 100);

    // vm.interpret();



    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[0])?;
    let mut contents = String::new();

    file.read_to_string(&mut contents);

    Ok(())

}