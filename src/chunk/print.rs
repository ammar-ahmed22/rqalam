use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct Print {
    code: OpCode,
}

impl Print {
    pub fn new() -> Self {
        return Self {
            code: OpCode::Print,
        };
    }
}

impl OperationBase for Print {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        _: usize,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        _: usize,
    ) -> Result<usize, QalamError> {
        let popped = stack.borrow_mut().pop().unwrap();
        println!("{}", popped);
        // stack.borrow_mut().push(self.operand.clone());

        return Ok(0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16}", "OP_PRINT")
    }
}
