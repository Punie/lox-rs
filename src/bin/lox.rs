use std::error::Error;

use lox::{
    bytecode::{Chunk, OpCode, Value},
    vm::Vm,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut chunk = Chunk::new();

    let value = chunk.push_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant(value), 123);

    let value = chunk.push_constant(Value::Number(3.4));
    chunk.write(OpCode::Constant(value), 123);

    chunk.write(OpCode::Add, 123);

    let value = chunk.push_constant(Value::Number(5.6));
    chunk.write(OpCode::Constant(value), 123);

    chunk.write(OpCode::Divide, 123);
    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Return, 123);

    let mut vm = Vm::init(chunk);
    vm.interpret()?;

    Ok(())
}
