use crate::opcode::OpCode;
use crate::chunk::Chunk;
use crate::disassembler::Disassembler;

pub struct Settings {
    debug: bool
}

pub struct VM{
    pub chunk: Chunk,
    pc: usize,
    settings: Settings
}

pub enum InterpretResult { 
    InterpretOk, 
    InterpretCompileError,
    InterpretRuntimeError
}

impl VM{
    pub fn new(chunk: Chunk) -> Self {
        VM { chunk:chunk , pc: 0 , settings: Settings { debug: true } }
    }
    pub fn update_chunk( &mut self, chunk: Chunk) {
        self.chunk = chunk;

    }
    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }  
    fn run(&mut self) -> InterpretResult { 
        loop {
            
            //debug instruction
            if self.settings.debug {
                Disassembler::disassemble_instruction(&self.chunk, self.pc); 
            }

            let instruction:OpCode = unsafe { std::mem::transmute(self.chunk.code[self.pc]) } ; 
            self.increment_counter();
              // incrementing pc here after getting the first value 
            match instruction {
                OpCode::OpConstant => {
                     // incrementing pc here after getting the value of self.chunk.code
                    let number = self.chunk.code[self.pc] as usize;
                    self.increment_counter();
                    let constant = self.chunk.constants[number];
                    println!("'{}'", constant);
              
                }
                OpCode::OpReturn => {
                    return InterpretResult::InterpretOk
                }
            }
      
        }

    }
    fn increment_counter(&mut self) {
        self.pc += 1;
    }
}