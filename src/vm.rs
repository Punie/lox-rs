use std::{error, fmt};

use crate::bytecode::{disassemble_instruction, Chunk, OpCode, Value};

#[derive(Clone, Copy, Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError,
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InterpretError::CompileError => write!(f, "compile error"),
            InterpretError::RuntimeError => write!(f, "runtime error"),
        }
    }
}

impl error::Error for InterpretError {}

pub type InterpretResult = Result<(), InterpretError>;

#[derive(Clone, Debug)]
pub struct Vm {
    chunk: Chunk,
    program_counter: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn init(chunk: Chunk) -> Self {
        Vm {
            chunk,
            program_counter: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            let instruction = self.chunk.code[self.program_counter];

            if cfg!(debug_assertions) {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {} ]", slot);
                }
                println!("");
                disassemble_instruction(&self.chunk, self.program_counter)
            }

            self.program_counter += 1;

            match instruction {
                OpCode::Constant(idx) => {
                    let constant = self.chunk.constants[idx];
                    self.stack.push(constant);
                }
                OpCode::Add => match (self.pop()?, self.pop()?) {
                    (Value::Number(b), Value::Number(a)) => self.stack.push(Value::Number(a + b)),
                },
                OpCode::Substract => match (self.pop()?, self.pop()?) {
                    (Value::Number(b), Value::Number(a)) => self.stack.push(Value::Number(a - b)),
                },
                OpCode::Multiply => match (self.pop()?, self.pop()?) {
                    (Value::Number(b), Value::Number(a)) => self.stack.push(Value::Number(a * b)),
                },
                OpCode::Divide => match (self.pop()?, self.pop()?) {
                    (Value::Number(b), Value::Number(a)) => self.stack.push(Value::Number(a / b)),
                },
                OpCode::Negate => {
                    let value = self.stack.pop().ok_or(InterpretError::RuntimeError)?;
                    match value {
                        Value::Number(val) => self.stack.push(Value::Number(-val)),
                    }
                }
                OpCode::Return => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value);
                    return Ok(());
                }
            }
        }
    }

    fn pop(&mut self) -> Result<Value, InterpretError> {
        self.stack.pop().ok_or(InterpretError::RuntimeError)
    }
}
