use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct Get {
    code: OpCode,
    operand: String,
}

impl Get {
    pub fn new(operand: String) -> Self {
        return Self {
            code: OpCode::Get,
            operand,
        };
    }
}

impl OperationBase for Get {
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
        let val = (*globals).borrow().get(&name);
        if let Some(val) = val {
         stack.borrow_mut().push(val); 
        } else {
          return Err(QalamError::with_line_runtime(&format!("Undefined variable '{}'.", name), line))
        }

        return Ok(0);
    }
}

impl Display for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_GET", self.operand)
    }
}
