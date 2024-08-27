pub mod binary;
pub mod constant;
pub mod operation;
pub mod return_op;
pub mod unary;
use operation::Operation;
use std::fmt::Display;

pub struct Chunk {
    pub code: Vec<Box<dyn Operation>>,
    capacity: usize,
    count: usize,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        return Self {
            code: Vec::new(),
            capacity: 0,
            count: 0,
            lines: Vec::new(),
        };
    }

    pub fn write(&mut self, op: Box<dyn Operation>, line: usize) {
        self.code.push(op);
        self.count += 1;
        self.capacity += 1;
        self.lines.push(line);
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let line = 1;
        let mut idx = 0;
        for inst in &self.code {
            if idx != 0 && line == self.lines[idx] {
                str = str + &format!("{}  {}\n", "|", inst);
            } else {
                str = str + &format!("{}  {}\n", self.lines[idx], inst);
            }
            idx += 1;
        }
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use crate::chunk::{constant::Constant, return_op::ReturnOp, Chunk};
    use crate::value::Value;

    #[test]
    fn test_chunk_display() {
        let mut chunk = Chunk::new();
        chunk.write(Box::new(Constant::new(Value::Number(1.2))), 1);
        chunk.write(Box::new(Constant::new(Value::Number(3.4))), 1);
        chunk.write(Box::new(ReturnOp::new()), 2);
        assert_eq!(
            format!("{}", chunk),
            "1  OP_CONSTANT      '1.2000'\n|  OP_CONSTANT      '3.4000'\n2  OP_RETURN\n"
        );
        print!("{}", chunk);
    }
}
