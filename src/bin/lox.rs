use lox::bytecode::{Chunk, OpCode, Value};

fn main() {
    let mut chunk = Chunk::new();
    let value = chunk.push_constant(Value::Number(42.0));
    chunk.write(OpCode::Constant(value), 123);
    chunk.write(OpCode::Return, 123);
    chunk.disassemble("test chunk");
}