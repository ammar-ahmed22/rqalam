use super::operation::{OpCode, OperationBase};
use crate::vm::table::Table;
use crate::{error::QalamError, value::Value};
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Mult,
    Div,
    Modulo,
    Equal,
    Greater,
    Less,
}

impl BinaryOp {
    pub fn eval(&self, a: Value, b: Value, line: usize) -> Result<Value, QalamError> {
        match self {
            Self::Add => {
                if let (Value::Number(a), Value::Number(b)) = (a.clone(), b.clone()) {
                    return Ok(Value::Number(a + b));
                } else if let (Value::String(a), Value::String(b)) = (a, b) {
                    return Ok(Value::String(a + &b));
                } else {
                    return Err(QalamError::with_line_runtime(
                        "Operands must be 2 numbers or 2 strings!",
                        line,
                    ));
                }
            }
            Self::Subtract => {
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Ok(Value::Number(a - b));
                } else {
                    return Err(QalamError::with_line_runtime(
                        "Operands must be numbers!",
                        line,
                    ));
                }
            }
            Self::Mult => {
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Ok(Value::Number(a * b));
                } else {
                    return Err(QalamError::with_line_runtime(
                        "Operands must be numbers!",
                        line,
                    ));
                }
            }
            Self::Div => {
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Ok(Value::Number(a / b));
                } else {
                    return Err(QalamError::with_line_runtime(
                        "Operands must be numbers!",
                        line,
                    ));
                }
            },
            Self::Modulo => {
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Ok(Value::Number(a % b))
                } else {
                    return Err(QalamError::with_line_runtime("Operands must be numbers!", line))
                }
            },
            Self::Equal => return Ok(Value::Bool(a == b)),
            Self::Greater => {
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Ok(Value::Bool(a > b));
                } else {
                    return Err(QalamError::with_line_runtime(
                        "Operands must be numbers!",
                        line,
                    ));
                }
            }
            Self::Less => {
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Ok(Value::Bool(a < b));
                } else {
                    return Err(QalamError::with_line_runtime(
                        "Operands must be numbers!",
                        line,
                    ));
                }
            }
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Mult => "*",
            BinaryOp::Div => "/",
            BinaryOp::Equal => "==",
            BinaryOp::Greater => ">",
            BinaryOp::Less => "<",
            BinaryOp::Modulo => "%"
        };
        write!(f, "{}", op_str)
    }
}

pub struct Binary {
    code: OpCode,
    op: BinaryOp,
}

impl Binary {
    pub fn new(op: BinaryOp) -> Self {
        return Self {
            code: OpCode::Binary,
            op,
        };
    }
}

impl OperationBase for Binary {
    fn disassemble(&self) -> OpCode {
        self.code.clone()
    }

    fn eval(
        &self,
        _: usize,
        stack: Rc<RefCell<Vec<Value>>>,
        _: Rc<RefCell<Vec<String>>>,
        _: Rc<RefCell<Table>>,
        line: usize,
    ) -> Result<usize, QalamError> {
        let b = stack.borrow_mut().pop().unwrap();
        let a = stack.borrow_mut().pop().unwrap();
        let val = self.op.eval(a, b, line)?;
        stack.borrow_mut().push(val);
        return Ok(0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<16} '{}'", "OP_BINARY", self.op)
    }
}
