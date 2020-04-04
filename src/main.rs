// since nesse is here I am going to give it another try.


/*
    name: Take 1
*/


pub mod chunk;
pub mod disassembler;
pub mod opcode;
pub mod vm;

use crate::chunk::Chunk;

use crate::opcode::OpCode;
use crate::vm::VM;



fn main() {
    let chunk = Chunk::new();
    let mut vm = VM::new(chunk);
    let constant: usize = vm.chunk.add_constant(1.2);
    vm.chunk.write_chunk(OpCode::OpConstant as u8, 100);
    vm.chunk.write_chunk(constant as u8, 100);
    vm.chunk.write_chunk(OpCode::OpReturn as u8, 100);

    vm.interpret();
    // disassembler.disassemble_chunk(&vm.chunk, String::from("test chunk"));

}