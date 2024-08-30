use crate::value::Value;
use crate::{error::QalamError, vm::table::Table};
use std::any::Any;
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
    Jump,
    FalseJump
}

pub trait OperationBase {
    fn disassemble(&self) -> OpCode;
    fn eval(
        &self,
        curr_offset: usize,
        stack: Rc<RefCell<Vec<Value>>>,
        call_frame: Rc<RefCell<Vec<String>>>,
        globals: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Operation: OperationBase + Display + Any {}
impl<T> Operation for T where T: Display + OperationBase + Any {}
