use crate::chunk::Chunk;
use crate::disassembler::Disassembler;
use crate::opcode::OpCode;

pub struct Settings {
    debug: bool,
}

pub struct VM {
    pub chunk: Chunk,
    pc: usize,
    settings: Settings,
    stack: Vec<f64>,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM {
            chunk: chunk,
            pc: 0,
            settings: Settings { debug: true },
            stack: Vec::new(),
        }
    }
    pub fn update_chunk(&mut self, chunk: Chunk) {
        self.chunk = chunk;
    }
    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }
    fn run(&mut self) -> InterpretResult {
        loop {
            //debug instruction
            if self.settings.debug {
                println!("    ");
                for x in self.stack.iter() {
                    print!("['{}']", x);
                }
                println!("\n");
                Disassembler::disassemble_instruction(&self.chunk, self.pc);
            }

            let instruction: OpCode = unsafe { std::mem::transmute(self.chunk.code[self.pc]) };
            self.increment_counter();
            // incrementing pc here after getting the first value
            match instruction {
                OpCode::OpAdd => self.binary("add"),
                OpCode::OpSub => self.binary("sub"),
                OpCode::OpMult => self.binary("mult"),
                OpCode::OpDiv => self.binary("divide"),
                OpCode::OpMod => self.binary("mod"),
                OpCode::OpConstant => {
                    // incrementing pc here after getting the value of self.chunk.code
                    let number = self.chunk.code[self.pc] as usize;
                    self.increment_counter();
                    let constant = self.chunk.constants[number];
                    self.stack.push(constant);
                }
                OpCode::OpNegate => {
                    let negate_num = self.stack.pop().unwrap();
                    self.stack.push(-negate_num);
                }
                OpCode::OpReturn => {
                    let x = self.stack.pop();
                    println!("'{}'", x.unwrap());
                    return InterpretResult::InterpretOk
                }
   
            }
        }
    }
    fn increment_counter(&mut self) {
        self.pc += 1;
    }
    fn binary(&mut self, opp: &str) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        match opp {
            "add" => self.stack.push(a + b),
            "sub" => self.stack.push(a - b),
            "mult" => self.stack.push(a * b),
            "divide" => self.stack.push(a / b),
            "mod" => self.stack.push(a % b),
            &_ => println!("unknown operator")
        }
    }
}
