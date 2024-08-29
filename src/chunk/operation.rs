use crate::value::Value;
use crate::{error::QalamError, vm::table::Table};
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Constant,
    Unary,
    Binary,
    Print,
    Pop,
    PopN,
    Define,
    Get,
    Set,
}

pub trait OperationBase {
    fn disassemble(&self) -> OpCode;
    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        globals: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError>;
}

pub trait Operation: OperationBase + Display {}
impl<T> Operation for T where T: Display + OperationBase {}
