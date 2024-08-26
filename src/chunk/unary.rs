use std::fmt::Display;
use crate::{error::QalamError, value::Value};
use super::operation::{ OpCode, OperationBase };

#[derive(Debug)]
pub enum UnaryOp {
  Negate,
  Bang
}

impl Display for UnaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let op_str = match self {
        UnaryOp::Negate => "-",
        UnaryOp::Bang => "!" 
      };
      write!(f, "{}", op_str)
  }
}


pub struct Unary {
  code: OpCode,
  op: UnaryOp
}

impl Unary {
  pub fn new(op: UnaryOp) -> Self {
    return Self {
      code: OpCode::Unary,
      op
    }
  }
}

impl OperationBase for Unary {
  fn disassemble(&self) -> OpCode {
      self.code.clone()
  }

  fn eval(&self, stack: std::rc::Rc<std::cell::RefCell<Vec<Value>>>, _: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> Result<usize, QalamError> {
      // stack.borrow_mut().push(self.operand.clone());
      let val = stack.borrow_mut().pop().unwrap();
      match self.op {
        UnaryOp::Negate => {
          if let Value::Number(val) = val {
            stack.borrow_mut().push(Value::Number(-val));
          } else {
            return Err(QalamError::new_compile("Can only negate numbers!"))
          }
        },
        _ => {}
      }
      return Ok(0);
  }
}

impl Display for Unary {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:<16} '{}'", "OP_UNARY", self.op)
  }
}