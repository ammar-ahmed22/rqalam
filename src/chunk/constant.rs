use std::fmt::Display;
use crate::{error::QalamError, value::Value};
use super::operation::{ OpCode, OperationBase };

pub struct Constant {
  code: OpCode,
  operand: Value
}

impl Constant {
  pub fn new(operand: Value) -> Self {
    return Self {
      code: OpCode::Constant,
      operand
    }
  }
}

impl OperationBase for Constant {
  fn disassemble(&self) -> OpCode {
      self.code.clone()
  }

  fn eval(&self, stack: std::rc::Rc<std::cell::RefCell<Vec<Value>>>, _: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> Result<usize, QalamError> {
      stack.borrow_mut().push(self.operand.clone());

      return Ok(0);
  }
}

impl Display for Constant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:<16} '{}'", "OP_CONSTANT", self.operand)
  }
}