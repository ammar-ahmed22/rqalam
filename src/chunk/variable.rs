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
        _: usize,
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for Define {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_DEFINE", self.operand)
    }
}

#[derive(Debug)]
pub enum Scope {
    Global,
    Local(usize),
}

pub struct Get {
    code: OpCode,
    operand: String,
    scope: Scope,
}

impl Get {
    pub fn new(operand: String, scope: Scope) -> Self {
        return Self {
            code: OpCode::Get,
            operand,
            scope,
        };
    }
}

impl OperationBase for Get {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        _: usize,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        globals: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError> {
        let name = self.operand.clone();
        match self.scope {
            Scope::Global => {
                let val = (*globals).borrow().get(&name);
                if let Some(val) = val {
                    stack.borrow_mut().push(val);
                } else {
                    return Err(QalamError::with_line_runtime(
                        &format!("Undefined variable '{}'.", name),
                        line,
                    ));
                }
            }
            Scope::Local(slot) => {
                let val = stack.borrow()[slot].clone();
                stack.borrow_mut().push(val);
            }
        }

        return Ok(0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_GET", self.operand)
    }
}

pub struct Set {
    code: OpCode,
    operand: String,
    scope: Scope,
}

impl Set {
    pub fn new(operand: String, scope: Scope) -> Self {
        return Self {
            code: OpCode::Set,
            operand,
            scope,
        };
    }
}

impl OperationBase for Set {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        _: usize,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        globals: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError> {
        let name = self.operand.clone();
        let stack_top = stack.borrow().len() - 1;
        let val = stack.borrow()[stack_top].clone();
        match self.scope {
            Scope::Global => match (*globals).borrow_mut().overwrite(name.clone(), val) {
                Some(_) => {}
                None => {
                    return Err(QalamError::with_line_runtime(
                        &format!("Undefined variable '{}'.", name),
                        line,
                    ))
                }
            },
            Scope::Local(slot) => {
                stack.borrow_mut()[slot] = val;
            }
        }

        return Ok(0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_SET", self.operand)
    }
}
