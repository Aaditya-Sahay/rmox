use crate::chunk::Chunk;
use crate::opcode::OpCode;

pub struct Disassembler {}

impl Disassembler {
    pub fn disassemble_chunk(chunk: &Chunk, name: String) {
        println!("== {} ==", name);

        let mut offset: usize = 0;
        while offset < chunk.code.len() {
            offset = Disassembler::disassemble_instruction(&chunk, offset);
        }
    }

    pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        print!("{} ", offset);

        if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            print!("  | ")
        } else {
            print!("{} ", chunk.lines[offset]);
        }

        let instruction: OpCode = unsafe { std::mem::transmute(chunk.code[offset]) };
        // TODO replace this later with a safe solution, right now it assumes that it is always going to be of Type Enum
        match instruction {
            OpCode::OpReturn => return Disassembler::simple_instruction("OpReturn", offset),
            OpCode::OpNegate => return Disassembler::simple_instruction("OpNegate", offset),
            OpCode::OpAdd => return Disassembler::simple_instruction("OpAdd", offset),
            OpCode::OpSub => return Disassembler::simple_instruction("OpSub", offset),
            OpCode::OpMult => return Disassembler::simple_instruction("OpMult", offset),
            OpCode::OpDiv => return Disassembler::simple_instruction("OpDiv", offset),
            OpCode::OpMod => return Disassembler::simple_instruction("OpMod",offset),
            OpCode::OpConstant => {
                return Disassembler::constant_instruction(&chunk, "OpConstant", offset)
            },
            // _ => {
            //     println!("Unknown opcode {}", instruction as u8);
            //     return offset + 1;
            // }
        }
    }

    pub fn constant_instruction(chunk: &Chunk, name: &str, offset: usize) -> usize {
        let constant: u8 = chunk.code[offset + 1];
        print!("{} {} ", name, constant);
        println!("'{}'", chunk.constants[constant as usize]);

        return offset + 2;
    }

    pub fn simple_instruction(opcode: &str, offset: usize) -> usize {
        println!("{}", opcode);
        return offset + 1;
    }
}
