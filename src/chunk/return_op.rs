use crate::error::QalamError;
use crate::value::Value;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::operation::{OpCode, OperationBase};

pub struct ReturnOp {
    code: OpCode,
}

impl ReturnOp {
    pub fn new() -> Self {
        return Self {
            code: OpCode::Return,
        };
    }
}

impl OperationBase for ReturnOp {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        _: Rc<RefCell<Vec<Value>>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        _: usize,
    ) -> Result<usize, QalamError> {
        call_frame.borrow_mut().pop();
        return Ok(0);
    }
}

impl Display for ReturnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OP_RETURN")
    }
}
