use super::operation::{OpCode, OperationBase};
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Bang,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            UnaryOp::Negate => "-",
            UnaryOp::Bang => "!",
        };
        write!(f, "{}", op_str)
    }
}

pub struct Unary {
    code: OpCode,
    op: UnaryOp,
}

impl Unary {
    pub fn new(op: UnaryOp) -> Self {
        return Self {
            code: OpCode::Unary,
            op,
        };
    }
}

impl OperationBase for Unary {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        line: usize,
    ) -> Result<usize, QalamError> {
        let val = stack.borrow_mut().pop().unwrap();
        match self.op {
            UnaryOp::Negate => {
                if let Value::Number(val) = val {
                    stack.borrow_mut().push(Value::Number(-val));
                } else {
                    return Err(QalamError::new_runtime(&format!(
                        "Operands must be numbers!\n\tat line {}",
                        line
                    )));
                }
            }
            UnaryOp::Bang => stack.borrow_mut().push(Value::Bool(val.is_falsy())),
        }
        return Ok(0);
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_UNARY", self.op)
    }
}
