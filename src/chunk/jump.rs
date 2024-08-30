use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub struct Jump {
    code: OpCode,
    pub jump: Option<usize>,
}

impl Jump {
    pub fn new() -> Self {
        Self {
            code: OpCode::Jump,
            jump: None,
        }
    }
}

impl OperationBase for Jump {
    fn disassemble(&self) -> OpCode {
        return self.code.clone();
    }

    fn eval(
        &self,
        curr_offset: usize,
        _: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError> {
        if let Some(jump) = self.jump {
            return Ok(curr_offset + jump + 1);
        } else {
            return Err(QalamError::with_line_compile("Jump was not patched!", line));
        }

        // return Ok(0);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{:?}'", "OP_JUMP", self.jump)
    }
}

pub struct FalseJump {
    code: OpCode,
    pub jump: Option<usize>,
}

impl FalseJump {
    pub fn new() -> Self {
        Self {
            code: OpCode::FalseJump,
            jump: None,
        }
    }
}

impl OperationBase for FalseJump {
    fn disassemble(&self) -> OpCode {
        return self.code.clone();
    }

    fn eval(
        &self,
        curr_offset: usize,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError> {
        if let Some(jump) = self.jump {
            let top = stack.borrow().len() - 1;
            let cond = stack.borrow()[top].clone();
            if cond.is_falsy() {
                return Ok(curr_offset + jump + 1);
            } else {
                return Ok(0);
            }
        } else {
            return Err(QalamError::with_line_compile("Jump was not patched!", line));
        }
        // todo!();
        // return Ok(0);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for FalseJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{:?}'", "OP_FALSE_JUMP", self.jump)
    }
}
