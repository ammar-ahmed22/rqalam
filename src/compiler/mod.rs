use parser::Parser;
use scanner::Scanner;

use crate::{chunk::Chunk, error::QalamError};

pub mod parser;
pub mod precedence;
pub mod scanner;
pub mod token;

pub struct Compiler {}

impl Compiler {
    pub fn compile(stream: Vec<u8>) -> Result<Chunk, QalamError> {
        let scanner = Scanner::new(stream);
        let mut chunk = Chunk::new();
        let parser = Parser::new(&scanner, &mut chunk)?;
        parser.parse()?;
        println!("{}", chunk);
        return Ok(chunk);
    }
}
