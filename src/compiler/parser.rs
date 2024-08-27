use std::cell::RefCell;

use crate::chunk::binary::Binary;
use crate::chunk::binary::BinaryOp;
use crate::chunk::constant::Constant;
use crate::chunk::operation::Operation;
use crate::chunk::return_op::ReturnOp;
use crate::chunk::unary::Unary;
use crate::chunk::unary::UnaryOp;
use crate::error::QalamError;
use crate::value::Value;

use super::precedence::Precedence;
use super::token::Token;
use super::token::TokenType;
use super::Chunk;
use super::Scanner;

pub struct Parser<'a> {
    scanner: &'a Scanner<'a>,
    chunk: RefCell<&'a mut Chunk>,
    current: RefCell<Token<'a>>,
    previous: RefCell<Option<Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: &'a Scanner<'a>, chunk: &'a mut Chunk) -> Result<Self, QalamError> {
        let curr = scanner.scan()?;
        return Ok(Self {
            scanner,
            chunk: RefCell::new(chunk),
            current: RefCell::new(curr),
            previous: RefCell::new(None),
        });
    }

    fn consume(&self, token_type: TokenType, message: &str) -> Result<(), QalamError> {
        if self.current.borrow().token_type == token_type {
            self.advance()?;
            return Ok(());
        }

        return Err(QalamError::from_token_compile(
            message,
            &self.current.borrow(),
        ));
    }

    pub fn literal(&self) -> Result<(), QalamError> {
        let prev = self.previous.borrow().as_ref().unwrap().clone();
        match prev.token_type {
            TokenType::FALSE => {
                self.emit_op(Constant::new(Value::Bool(false)));
            }
            TokenType::TRUE => {
                self.emit_op(Constant::new(Value::Bool(true)));
            }
            TokenType::NIL => {
                self.emit_op(Constant::new(Value::Null));
            }
            TokenType::STRING => {
                if let Some((_, rest)) = prev.literal.split_first() {
                    if let Some((_, mid)) = rest.split_last() {
                        let string = std::str::from_utf8(mid).unwrap().to_string();
                        self.emit_op(Constant::new(Value::String(string)));
                    }
                }
                // let string = std::str::from_utf8(&prev.literal).unwrap().to_string();
                // self.emit_op(Constant::new(Value::String(string)));
            }
            _ => {}
        };
        return Ok(());
    }

    pub fn grouping(&self) -> Result<(), QalamError> {
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
        return Ok(());
    }

    pub fn unary(&self) -> Result<(), QalamError> {
        let op_type = self.previous.borrow().as_ref().unwrap().clone().token_type;

        self.parse_precedence(Precedence::Unary)?;

        match op_type {
            TokenType::MINUS => self.emit_op(Unary::new(UnaryOp::Negate)),
            TokenType::BANG => self.emit_op(Unary::new(UnaryOp::Bang)),
            _ => {}
        };
        return Ok(());
    }

    pub fn binary(&self) -> Result<(), QalamError> {
        let op_type = self.previous.borrow().as_ref().unwrap().clone().token_type;
        let rule = Precedence::get_rule(op_type.clone());

        let next_prec = rule.precedence + 1;
        self.parse_precedence(next_prec.unwrap())?;

        match op_type {
            TokenType::PLUS => {
                self.emit_op(Binary::new(BinaryOp::Add));
            }
            TokenType::MINUS => {
                self.emit_op(Binary::new(BinaryOp::Subtract));
            }
            TokenType::STAR => {
                self.emit_op(Binary::new(BinaryOp::Mult));
            }
            TokenType::SLASH => {
                self.emit_op(Binary::new(BinaryOp::Div));
            }
            TokenType::BANG_EQUAL => {
                self.emit_op(Binary::new(BinaryOp::Equal));
                self.emit_op(Unary::new(UnaryOp::Bang));
            }
            TokenType::EQUAL_EQUAL => {
                self.emit_op(Binary::new(BinaryOp::Equal));
            }
            TokenType::GREATER => {
                self.emit_op(Binary::new(BinaryOp::Greater));
            }
            TokenType::GREATER_EQUAL => {
                self.emit_op(Binary::new(BinaryOp::Less));
                self.emit_op(Unary::new(UnaryOp::Bang));
            }
            TokenType::LESS => {
                self.emit_op(Binary::new(BinaryOp::Less));
            }
            TokenType::LESS_EQUAL => {
                self.emit_op(Binary::new(BinaryOp::Greater));
                self.emit_op(Unary::new(UnaryOp::Bang));
            }
            _ => {}
        };
        return Ok(());
    }

    fn parse_precedence(&self, precedence: Precedence) -> Result<(), QalamError> {
        self.advance()?;
        let prev = self.previous.borrow().as_ref().unwrap().clone();
        let prefix_rule = Precedence::get_rule(prev.token_type.clone()).prefix;
        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self)?;
        } else {
            return Err(QalamError::from_token_compile("Expect expression.", &prev));
        }

        while precedence
            <= Precedence::get_rule(self.current.borrow().clone().token_type).precedence
        {
            self.advance()?;
            let prev = self.previous.borrow().as_ref().unwrap().clone();
            let infix_rule = Precedence::get_rule(prev.token_type.clone()).infix;
            if let Some(infix_rule) = infix_rule {
                infix_rule(self)?;
            }
        }

        return Ok(());
    }

    fn advance(&self) -> Result<(), QalamError> {
        let next = self.scanner.scan()?;
        self.previous
            .replace_with(|_| Some(self.current.replace(next)));
        return Ok(());
    }

    fn emit_op(&self, op: impl Operation + 'static) {
        let mut chunk = self.chunk.borrow_mut();
        chunk.write(Box::new(op), self.previous.borrow().as_ref().unwrap().line);
    }

    fn emit_return(&self) {
        self.emit_op(ReturnOp::new());
    }

    fn expression(&self) -> Result<(), QalamError> {
        self.parse_precedence(Precedence::Assignment)?;
        return Ok(());
    }

    pub fn number(&self) -> Result<(), QalamError> {
        let prev = self.previous.borrow().as_ref().unwrap().clone();
        match std::str::from_utf8(&prev.literal).unwrap().parse::<f64>() {
            Ok(num) => self.emit_op(Constant::new(Value::Number(num))),
            Err(_) => {
                return Err(QalamError::from_token_compile("Invalid number.", &prev));
            }
        }

        return Ok(());
    }

    pub fn parse(&self) -> Result<(), QalamError> {
        self.expression()?;
        self.consume(TokenType::EOF, "Expect end of expression.")?;
        self.emit_return();
        return Ok(());
    }
}
