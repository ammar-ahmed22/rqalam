use std::fmt::Display;
use crate::{error::QalamError, value::Value};
use super::operation::{ OpCode, OperationBase };

#[derive(Debug)]
pub enum BinaryOp {
  Add,
  Subtract,
  Mult,
  Div,
}

impl BinaryOp {
  pub fn eval(&self, a: Value, b: Value) -> Result<Value, QalamError> {
    match self {
      Self::Add => {
        if let (Value::Number(a), Value::Number(b)) = (a, b) {
         return Ok(Value::Number(a + b));
        } else {
          return Err(QalamError::new_compile("Can only add 2 numbers!"));
        }
      },
      Self::Subtract => {
        if let (Value::Number(a), Value::Number(b)) = (a, b) {
          return Ok(Value::Number(a - b));
         } else {
           return Err(QalamError::new_compile("Can only subtract 2 numbers!"));
         }
      },
      Self::Mult => {
        if let (Value::Number(a), Value::Number(b)) = (a, b) {
          return Ok(Value::Number(a * b));
         } else {
           return Err(QalamError::new_compile("Can only multiply 2 numbers!"));
         }
      },
      Self::Div => {
        if let (Value::Number(a), Value::Number(b)) = (a, b) {
          return Ok(Value::Number(a / b));
         } else {
           return Err(QalamError::new_compile("Can only divide 2 numbers!"));
         }
      }
    }
  }
}

impl Display for BinaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let op_str = match self {
        BinaryOp::Add => "+",
        BinaryOp::Subtract => "-",
        BinaryOp::Mult => "*",
        BinaryOp::Div => "/" 
      };
      write!(f, "{}", op_str)
  }
}


pub struct Binary {
  code: OpCode,
  op: BinaryOp
}

impl Binary {
  pub fn new(op: BinaryOp) -> Self {
    return Self {
      code: OpCode::Binary,
      op
    }
  }
}

impl OperationBase for Binary {
  fn disassemble(&self) -> OpCode {
      self.code.clone()
  }

  fn eval(&self, stack: std::rc::Rc<std::cell::RefCell<Vec<Value>>>, _: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> Result<usize, QalamError> {
      // stack.borrow_mut().push(self.operand.clone());
      let b = stack.borrow_mut().pop().unwrap();
      let a = stack.borrow_mut().pop().unwrap();
      let val = self.op.eval(a, b)?;
      stack.borrow_mut().push(val);
      // match self.op {
      //   UnaryOp::Negate => {
      //     if let Value::Number(val) = val {
      //       stack.borrow_mut().push(Value::Number(-val));
      //     } else {
      //       return Err(QalamError::new_compile("Can only negate numbers!"))
      //     }
      //   },
      //   _ => {}
      // }
      return Ok(0);
  }
}

impl Display for Binary {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:<16} '{}'", "OP_BINARY", self.op)
  }
}