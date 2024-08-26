use std::fmt::Display;
use crate::error::QalamError;

use super::operation::{ OpCode, OperationBase };

pub struct ReturnOp {
  code: OpCode
}

impl ReturnOp {
  pub fn new() -> Self {
    return Self {
      code: OpCode::Return
    }
  }
}

impl OperationBase for ReturnOp {
  fn disassemble(&self) -> OpCode {
      self.code.clone()
  }

  fn eval(&self, _: std::rc::Rc<std::cell::RefCell<Vec<crate::value::Value>>>, call_frame: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> Result<usize, QalamError> {
      call_frame.borrow_mut().pop();
      return Ok(0);
  }
}

impl Display for ReturnOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "OP_RETURN")
  }
}