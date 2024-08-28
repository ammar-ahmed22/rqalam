use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct Define {
    code: OpCode,
    operand: String,
}

impl Define {
    pub fn new(operand: String) -> Self {
        return Self {
            code: OpCode::Define,
            operand,
        };
    }
}

impl OperationBase for Define {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        globals: Rc<RefCell<Table>>,
        _: usize,
    ) -> Result<usize, QalamError> {
        let name = self.operand.clone();
        let curr_idx = || {
            if stack.borrow().len() > 0 {
                return (*stack).borrow().len() - 1;
            }
            return 0;
        };
        (*globals)
            .borrow_mut()
            .add(name, stack.borrow()[curr_idx()].clone());
        stack.borrow_mut().pop();

        return Ok(0);
    }
}

impl Display for Define {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_DEFINE", self.operand)
    }
}
