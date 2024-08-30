use crate::error::QalamError;
use crate::value::Value;
use crate::vm::table::Table;
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
        _: usize,
        _: Rc<RefCell<Vec<Value>>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        _: usize,
    ) -> Result<usize, QalamError> {
        call_frame.borrow_mut().pop();
        return Ok(0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for ReturnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OP_RETURN")
    }
}
