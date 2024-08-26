use std::{cell::RefCell, fmt::Display, rc::Rc};
use crate::value::Value;
use crate::error::QalamError;

#[derive(Debug, Clone)]
pub enum OpCode {
  Return,
  Constant,
  Unary,
  Binary
}

pub trait OperationBase {
  fn disassemble(&self) -> OpCode;
  fn eval(&self, stack: Rc<RefCell<Vec<Value>>>, call_frame: Rc<RefCell<Vec<String>>>) -> Result<usize, QalamError>;
}

pub trait Operation: OperationBase + Display {}
impl <T> Operation for T where T: Display + OperationBase {}