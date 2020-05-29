use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    Constant(usize),
    Add,
    Substract,
    Multiply,
    Divide,
    Negate,
    Return,
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::Number(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, op_code: OpCode, line: usize) {
        self.code.push(op_code);
        self.lines.push(line);
    }

    pub fn code_at(&self, offset: usize) -> Option<OpCode> {
        self.code.get(offset).cloned()
    }

    pub fn constant_at(&self, offset: usize) -> Option<Value> {
        self.constants.get(offset).cloned()
    }

    pub fn push_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} == ", name);

        for offset in 0..self.code.len() {
            disassemble_instruction(self, offset);
        }
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    match chunk.code[offset] {
        OpCode::Constant(offset) => println!(
            "{:-16} {:4} '{}'",
            "OP_CONSTANT", offset, chunk.constants[offset]
        ),
        OpCode::Add => println!("OP_ADD"),
        OpCode::Substract => println!("OP_SUBSTRACT"),
        OpCode::Multiply => println!("OP_MULTIPLY"),
        OpCode::Divide => println!("OP_DIVIDE"),
        OpCode::Negate => println!("OP_NEGATE"),
        OpCode::Return => println!("OP_RETURN"),
    }
}
