use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct Set {
    code: OpCode,
    operand: String,
}

impl Set {
    pub fn new(operand: String) -> Self {
        return Self {
            code: OpCode::Set,
            operand,
        };
    }
}

impl OperationBase for Set {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        globals: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError> {
      let name = self.operand.clone();
      let curr_idx = || {
          if stack.borrow().len() > 0 {
              return (*stack).borrow().len() - 1;
          }
          return 0;
      };
      match (*globals).borrow_mut().overwrite(name.clone(), stack.borrow()[curr_idx()].clone()) {
        Some(_) => {},
        None => {
          return Err(QalamError::with_line_runtime(&format!("Undefined variable '{}'.", name), line))
        }
      }

        return Ok(0);
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_SET", self.operand)
    }
}
