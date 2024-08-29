// use rqalam::chunk::{ Chunk, return_op::ReturnOp, constant::Constant, unary::{ Unary, UnaryOp }, binary::{ Binary, BinaryOp } };
// use rqalam::value::Value;
use rqalam::{error::QalamError, vm::VM};

use std::io::Write;

pub fn repl() -> Result<(), QalamError> {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("Could not flush!");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "exit()\n" {
                    break;
                }
                let stream = Vec::<u8>::from(input.clone());
                let mut vm = VM::new();
                vm.interpret(stream)?;
                input.clear();
            }
            Err(e) => {
                return Err(QalamError::new_runtime(&format!("{}", e)));
            }
        }
    }

    return Ok(());
}

pub fn source(path: &String) -> Result<(), QalamError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            let stream = Vec::<u8>::from(contents.clone() + "\n");
            let mut vm = VM::new();
            vm.interpret(stream)?;
            return Ok(());
        }
        Err(e) => return Err(QalamError::new_compile(&format!("{}", e))),
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        if let Err(e) = repl() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    } else if args.len() == 2 {
        // source
        if let Err(e) = source(&args[1]) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Usage: rqalam [path]");
        std::process::exit(1);
    }
}
