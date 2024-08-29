use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct Pop {
    code: OpCode,
}

impl Pop {
    pub fn new() -> Self {
        return Self { code: OpCode::Pop };
    }
}

impl OperationBase for Pop {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        _: usize,
    ) -> Result<usize, QalamError> {
        stack.borrow_mut().pop().unwrap();
        return Ok(0);
    }
}

impl Display for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16}", "OP_POP")
    }
}

pub struct PopN {
    code: OpCode,
    n: usize,
}

impl PopN {
    pub fn new(n: usize) -> Self {
        return Self {
            code: OpCode::Pop,
            n,
        };
    }
}

impl OperationBase for PopN {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        _: usize,
    ) -> Result<usize, QalamError> {
        for _ in 0..self.n {
            stack.borrow_mut().pop().unwrap();
        }
        return Ok(0);
    }
}

impl Display for PopN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_POPN", self.n)
    }
}
