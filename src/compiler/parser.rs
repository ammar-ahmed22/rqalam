use std::cell::RefCell;

use crate::chunk::binary::Binary;
use crate::chunk::binary::BinaryOp;
use crate::chunk::constant::Constant;
use crate::chunk::jump::FalseJump;
use crate::chunk::jump::Jump;
use crate::chunk::jump::LoopJump;
use crate::chunk::operation::Operation;
use crate::chunk::pop::Pop;
use crate::chunk::print::Print;
use crate::chunk::return_op::ReturnOp;
use crate::chunk::unary::Unary;
use crate::chunk::unary::UnaryOp;
use crate::chunk::variable::Define;
use crate::chunk::variable::Get;
use crate::chunk::variable::Set;
use crate::error::QalamError;
use crate::value::Value;

use super::precedence::Precedence;
use super::token::Token;
use super::token::TokenType;
use super::Chunk;
use super::Compiler;
use super::Scanner;

pub struct Parser<'a> {
    scanner: &'a Scanner<'a>,
    chunk: RefCell<&'a mut Chunk>,
    current: RefCell<Token<'a>>,
    previous: RefCell<Option<Token<'a>>>,
    compiler: RefCell<&'a mut Compiler>,
}

impl<'a> Parser<'a> {
    pub fn new(
        scanner: &'a Scanner<'a>,
        chunk: &'a mut Chunk,
        compiler: &'a mut Compiler,
    ) -> Result<Self, QalamError> {
        let curr = scanner.scan()?;
        return Ok(Self {
            scanner,
            chunk: RefCell::new(chunk),
            current: RefCell::new(curr),
            previous: RefCell::new(None),
            compiler: RefCell::new(compiler),
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

    pub fn literal(&self, _: bool) -> Result<(), QalamError> {
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

    pub fn grouping(&self, _: bool) -> Result<(), QalamError> {
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;
        return Ok(());
    }

    pub fn unary(&self, _: bool) -> Result<(), QalamError> {
        let op_type = self.previous.borrow().as_ref().unwrap().clone().token_type;

        self.parse_precedence(Precedence::Unary)?;

        match op_type {
            TokenType::MINUS => self.emit_op(Unary::new(UnaryOp::Negate)),
            TokenType::BANG => self.emit_op(Unary::new(UnaryOp::Bang)),
            _ => {}
        };
        return Ok(());
    }

    pub fn binary(&self, _: bool) -> Result<(), QalamError> {
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
            TokenType::PERCENT => {
                self.emit_op(Binary::new(BinaryOp::Modulo));
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
            let can_assign = precedence <= Precedence::Assignment;
            prefix_rule(self, can_assign)?;
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
                let can_assign = precedence <= Precedence::Assignment;
                infix_rule(self, can_assign)?;
                if can_assign && self.match_token(TokenType::EQUAL)? {
                    return Err(QalamError::from_token_compile(
                        "Invalid assignment target.",
                        self.previous.clone().borrow().as_ref().unwrap(),
                    ));
                }
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

    pub fn number(&self, _: bool) -> Result<(), QalamError> {
        let prev = self.previous.borrow().as_ref().unwrap().clone();
        match std::str::from_utf8(&prev.literal).unwrap().parse::<f64>() {
            Ok(num) => self.emit_op(Constant::new(Value::Number(num))),
            Err(_) => {
                return Err(QalamError::from_token_compile("Invalid number.", &prev));
            }
        }

        return Ok(());
    }

    fn check_token(&self, token_type: TokenType) -> bool {
        return self.current.borrow().token_type == token_type;
    }

    fn match_token(&self, token_type: TokenType) -> Result<bool, QalamError> {
        if !self.check_token(token_type) {
            return Ok(false);
        }
        self.advance()?;
        return Ok(true);
    }

    fn match_tokens(&self, token_types: &[TokenType]) -> Result<bool, QalamError> {
        for token_type in token_types {
            if self.match_token(token_type.clone())? {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    fn print_statement(&self) -> Result<(), QalamError> {
        self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        self.emit_op(Print::new());
        return Ok(());
    }

    fn expression_statement(&self) -> Result<(), QalamError> {
        self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        self.emit_op(Pop::new());
        return Ok(());
    }

    pub fn block(&self) -> Result<(), QalamError> {
        while !self.check_token(TokenType::RIGHT_BRACE) && !self.check_token(TokenType::EOF) {
            self.declaration()?;
        }

        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        return Ok(());
    }

    fn emit_jump(&self, op: impl Operation + 'static) -> usize {
        self.emit_op(op);
        return self.chunk.borrow().count - 1;
    }

    fn patch_false_jump(&self, jump: usize) {
        let offset = self.chunk.borrow().count - 1 - jump; // how much to jump
        let mut chunk = self.chunk.borrow_mut();

        let jump = &mut chunk.code[jump];
        let false_jump_op = jump.as_any_mut().downcast_mut::<FalseJump>();
        if let Some(false_jump_op) = false_jump_op {
            false_jump_op.jump = Some(offset);
        }
    }

    fn patch_jump(&self, jump: usize) {
        let offset = self.chunk.borrow().count - 1 - jump; // how much to jump
        let mut chunk = self.chunk.borrow_mut();

        let jump = &mut chunk.code[jump];
        let jump_op = jump.as_any_mut().downcast_mut::<Jump>();
        if let Some(jump_op) = jump_op {
            jump_op.jump = Some(offset);
        }
    }

    pub fn and(&self, _: bool) -> Result<(), QalamError> {
        let end_jump = self.emit_jump(FalseJump::new());
        self.emit_op(Pop::new());

        self.parse_precedence(Precedence::And)?;
        self.patch_false_jump(end_jump);
        return Ok(());
    }

    pub fn or(&self, _: bool) -> Result<(), QalamError> {
        let else_jump = self.emit_jump(FalseJump::new());
        let end_jump = self.emit_jump(Jump::new());
        self.patch_false_jump(else_jump);
        self.emit_op(Pop::new());
        self.parse_precedence(Precedence::Or)?;
        self.patch_jump(end_jump);
        return Ok(());
    }

    fn if_statement(&self) -> Result<(), QalamError> {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'itha'.")?;
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.")?;

        let then_jump = self.emit_jump(FalseJump::new());
        self.emit_op(Pop::new());
        self.statement()?;
        let else_jump = self.emit_jump(Jump::new());

        self.patch_false_jump(then_jump);
        self.emit_op(Pop::new());
        if self.match_token(TokenType::ELSE)? {
            self.statement()?;
        }
        self.patch_jump(else_jump);

        // self.emit_op(Pop::new());
        return Ok(());
    }

    fn emit_loop(&self, start: usize) {
        let jump = self.chunk.borrow().count - 1 - start;
        self.emit_op(LoopJump::new(jump));
    }

    fn while_statement(&self) -> Result<(), QalamError> {
        let loop_start = self.chunk.borrow().count - 1;
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'baynama'.")?;
        self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.")?;

        let exit_jump = self.emit_jump(FalseJump::new());
        self.emit_op(Pop::new());
        self.statement()?;
        self.emit_loop(loop_start);
        self.patch_false_jump(exit_jump);
        self.emit_op(Pop::new());
        return Ok(());
    }

    fn for_statement(&self) -> Result<(), QalamError> {
        self.compiler.borrow_mut().begin_scope();
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'tawaf'.")?;
        // self.consume(TokenType::SEMICOLON, "Expect ';'.")?;
        if self.match_token(TokenType::SEMICOLON)? {
            // no initializer
        } else if self.match_token(TokenType::VAR)? {
            self.var_declaration(false)?;
        } else {
            self.expression_statement()?;
        }

        let mut loop_start = self.chunk.borrow().count - 1;
        let mut exit_jump = None;
        if !self.match_token(TokenType::SEMICOLON)? {
            self.expression()?;
            self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;
            exit_jump = Some(self.emit_jump(FalseJump::new()));
            self.emit_op(Pop::new());
        }
        // self.consume(TokenType::SEMICOLON, "Expect ';'.")?;

        if !self.match_token(TokenType::RIGHT_PAREN)? {
            let body_jump = self.emit_jump(Jump::new());
            let inc_start = self.chunk.borrow().count - 1;
            self.expression()?;
            self.emit_op(Pop::new());
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after tawaf clauses.")?;
            self.emit_loop(loop_start);
            loop_start = inc_start;
            self.patch_jump(body_jump);
        }

        self.statement()?;

        self.emit_loop(loop_start);
        if let Some(exit_jump) = exit_jump {
            self.patch_false_jump(exit_jump);
            self.emit_op(Pop::new());
        }
        self.compiler.borrow_mut().end_scope(
            &mut self.chunk.borrow_mut(),
            self.previous.clone().borrow().as_ref().unwrap().line,
        );
        return Ok(());
    }

    pub fn statement(&self) -> Result<(), QalamError> {
        if self.match_token(TokenType::PRINT)? {
            self.print_statement()?;
        } else if self.match_token(TokenType::FOR)? {
            self.for_statement()?;
        } else if self.match_token(TokenType::IF)? {
            self.if_statement()?;
        } else if self.match_token(TokenType::WHILE)? {
            self.while_statement()?;
        } else if self.match_token(TokenType::LEFT_BRACE)? {
            self.compiler.borrow_mut().begin_scope();
            self.block()?;
            self.compiler.borrow_mut().end_scope(
                &mut self.chunk.borrow_mut(),
                self.previous.clone().borrow().as_ref().unwrap().line,
            );
        } else {
            self.expression_statement()?;
        }

        return Ok(());
    }

    fn identifier_string(&self, name: Token) -> Result<String, QalamError> {
        return Ok(std::str::from_utf8(name.literal).unwrap().to_string());
    }

    fn declare_variable(&self, immutable: bool) -> Result<(), QalamError> {
        if self.compiler.borrow().scope_depth == 0 {
            return Ok(());
        }
        let prev = self.previous.clone().borrow().as_ref().unwrap().clone();
        let name = std::str::from_utf8(&prev.literal).unwrap().to_string();
        {
            let compiler = self.compiler.borrow();
            for i in (0..compiler.local_count).rev() {
                let local = &compiler.locals.borrow()[i];
                if local.init && local.depth < compiler.scope_depth {
                    break;
                }
                if name == local.name {
                    return Err(QalamError::from_token_compile(
                        "Already a variable with this name in this scope.",
                        &prev,
                    ));
                }
            }
        }

        self.compiler.borrow_mut().add_local(name, immutable);
        return Ok(());
    }

    fn parse_variable(&self, immutable: bool) -> Result<String, QalamError> {
        self.consume(TokenType::IDENTIFIER, "Expect variable name.")?;
        self.declare_variable(immutable)?;
        if self.compiler.borrow().scope_depth > 0 {
            return Ok(String::new());
        }
        return self.identifier_string(self.previous.borrow().as_ref().unwrap().clone());
    }

    fn named_variable(&self, name: Token, can_assign: bool) -> Result<(), QalamError> {
        let id = self.identifier_string(name)?;
        let (scope, immutable) = self.compiler.borrow().resolve_local(
            id.clone(),
            self.previous.clone().borrow().as_ref().unwrap().line,
        )?;

        if can_assign
            && self.match_tokens(&[
                TokenType::EQUAL,
                TokenType::INCREMENT,
                TokenType::DECREMENT,
                TokenType::PLUS_EQUAL,
                TokenType::MINUS_EQUAL,
                TokenType::STAR_EQUAL,
                TokenType::SLASH_EQUAL,
            ])?
        {
            if immutable {
                return Err(QalamError::from_token_compile(
                    &format!(
                        "Invalid assignment target. Cannot assign to 'lazim' variable '{}'.",
                        id
                    ),
                    self.previous.clone().borrow().as_ref().unwrap(),
                ));
            }
            match self.previous.clone().borrow().as_ref().unwrap().token_type {
                TokenType::EQUAL => {
                    self.expression()?;
                }
                TokenType::INCREMENT => {
                    self.emit_op(Get::new(id.clone(), scope.clone()));
                    self.emit_op(Constant::new(Value::Number(1.0)));
                    self.emit_op(Binary::new(BinaryOp::Add));
                }
                TokenType::DECREMENT => {
                    self.emit_op(Get::new(id.clone(), scope.clone()));
                    self.emit_op(Constant::new(Value::Number(1.0)));
                    self.emit_op(Binary::new(BinaryOp::Subtract));
                }
                TokenType::PLUS_EQUAL => {
                    self.emit_op(Get::new(id.clone(), scope.clone()));
                    self.expression()?;
                    self.emit_op(Binary::new(BinaryOp::Add))
                }
                TokenType::MINUS_EQUAL => {
                    self.emit_op(Get::new(id.clone(), scope.clone()));
                    self.expression()?;
                    self.emit_op(Binary::new(BinaryOp::Subtract))
                }
                TokenType::STAR_EQUAL => {
                    self.emit_op(Get::new(id.clone(), scope.clone()));
                    self.expression()?;
                    self.emit_op(Binary::new(BinaryOp::Mult))
                }
                TokenType::SLASH_EQUAL => {
                    self.emit_op(Get::new(id.clone(), scope.clone()));
                    self.expression()?;
                    self.emit_op(Binary::new(BinaryOp::Div))
                }
                _ => {}
            };
            self.emit_op(Set::new(id, scope));
        } else {
            self.emit_op(Get::new(id, scope));
        }
        return Ok(());
    }

    pub fn variable(&self, can_assign: bool) -> Result<(), QalamError> {
        self.named_variable(
            self.previous.clone().borrow().as_ref().unwrap().clone(),
            can_assign,
        )?;
        return Ok(());
    }

    fn define_variable(&self, name: String) -> Result<(), QalamError> {
        if self.compiler.borrow().scope_depth > 0 {
            self.compiler.borrow_mut().mark_initialized();
            return Ok(());
        }
        self.emit_op(Define::new(name));
        return Ok(());
    }

    pub fn var_declaration(&self, immutable: bool) -> Result<(), QalamError> {
        let global = self.parse_variable(immutable)?;

        if self.match_token(TokenType::EQUAL)? {
            self.expression()?;
        } else {
            self.emit_op(Constant::new(Value::Null))
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        // define_variable
        self.define_variable(global)?;
        return Ok(());
    }

    pub fn declaration(&self) -> Result<(), QalamError> {
        if self.match_token(TokenType::VAR)? {
            self.var_declaration(false)?
        } else if self.match_token(TokenType::CONST)? {
            self.var_declaration(true)?
        } else {
            self.statement()?;
        }

        return Ok(());
    }

    pub fn parse(&self) -> Result<(), QalamError> {
        self.compiler.borrow_mut().begin_scope();
        while !self.match_token(TokenType::EOF)? {
            self.declaration()?;
        }
        self.compiler.borrow_mut().end_scope(
            &mut self.chunk.borrow_mut(),
            self.previous.clone().borrow().as_ref().unwrap().line,
        );
        // self.consume(TokenType::EOF, "Expect end of expression.")?;
        self.emit_return();

        return Ok(());
    }
}
