use rqalam::chunk::{ Chunk, return_op::ReturnOp, constant::Constant, unary::{ Unary, UnaryOp }, binary::{ Binary, BinaryOp } };
use rqalam::value::Value;
use rqalam::vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Box::new(Constant::new(Value::Number(1.2))), 1);
    chunk.write(Box::new(Constant::new(Value::Number(3.4))), 1);
    chunk.write(Box::new(Binary::new(BinaryOp::Add)), 1);

    chunk.write(Box::new(Constant::new(Value::Number(5.6))), 1);
    chunk.write(Box::new(Binary::new(BinaryOp::Div)), 1);
    chunk.write(Box::new(Unary::new(UnaryOp::Negate)), 1);
    
    chunk.write(Box::new(ReturnOp::new()), 1);

    println!("== code ==");
    print!("{}", chunk);

    println!("== VM Execution ==");
    let mut vm = VM::new();
    vm.interpret(&mut chunk);
}
