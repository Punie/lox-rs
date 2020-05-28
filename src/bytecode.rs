use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    Constant(usize),
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

#[derive(Clone, Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
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

    pub fn push_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} == ", name);

        for (idx, op_code) in self.code.iter().enumerate() {
            print!("{:04} ", idx);

            if idx > 0 && self.lines[idx] == self.lines[idx - 1] {
                print!("   | ");
            } else {
                print!("{:4} ", self.lines[idx]);
            }

            match *op_code {
                OpCode::Return => println!("OP_RETURN"),
                OpCode::Constant(idx) => println!("{:-16} {:4} '{}'", "OP_CONSTANT", idx, self.constants[idx]),
            }
        }
    }
}
