use std::{convert::From, error, fmt};

use crate::bytecode::{disassemble_instruction, Chunk, OpCode, Value};

#[derive(Clone, Copy, Debug)]
pub enum InterpretError {
    CompileError,
    RuntimeError(RuntimeError),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InterpretError::CompileError => write!(f, "compile error"),
            InterpretError::RuntimeError(err) => write!(f, "runtime error: {}", err),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RuntimeError {
    InvalidChunkError,
    StackUnderflow,
    TypeError,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RuntimeError::InvalidChunkError => write!(f, "malformed chunk"),
            RuntimeError::StackUnderflow => write!(f, "stack underflow"),
            RuntimeError::TypeError => write!(f, "type error"),
        }
    }
}

impl From<RuntimeError> for InterpretError {
    fn from(error: RuntimeError) -> InterpretError {
        InterpretError::RuntimeError(error)
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
                println!();
                disassemble_instruction(&self.chunk, self.program_counter)
            }

            self.program_counter += 1;

            macro_rules! binary_op {
                ($op:tt) => {{
                    match (self.pop_stack()?, self.pop_stack()?) {
                        (Value::Number(b), Value::Number(a)) => self.push_stack(Value::Number(a $op b)),
                    }
                }};
            }

            match instruction {
                OpCode::Constant(idx) => {
                    let constant = self
                        .chunk
                        .constant_at(idx)
                        .ok_or(RuntimeError::InvalidChunkError)?;
                    self.push_stack(constant);
                }
                OpCode::Add => binary_op!(+),
                OpCode::Substract => binary_op!(-),
                OpCode::Multiply => binary_op!(*),
                OpCode::Divide => binary_op!(/),
                OpCode::Negate => {
                    let value = self.pop_stack()?;
                    match value {
                        Value::Number(val) => self.push_stack(Value::Number(-val)),
                    }
                }
                OpCode::Return => {
                    let value = self.pop_stack()?;
                    println!("{}", value);
                    return Ok(());
                }
            }
        }
    }

    fn pop_stack(&mut self) -> Result<Value, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }
}
