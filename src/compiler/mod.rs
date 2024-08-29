use std::{cell::RefCell, rc::Rc};

use parser::Parser;
use scanner::Scanner;

use crate::{
    chunk::{pop::PopN, variable::Scope, Chunk},
    error::QalamError,
};

pub mod parser;
pub mod precedence;
pub mod scanner;
pub mod token;

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    pub init: bool,
    pub immutable: bool,
}

impl Local {
    pub fn new(name: String, depth: usize, init: bool, immutable: bool) -> Self {
        Self { name, depth, init, immutable }
    }
}

pub struct Compiler {
    pub locals: Rc<RefCell<Vec<Local>>>,
    pub local_count: usize,
    pub scope_depth: usize,
}

impl Compiler {
    pub fn compile(stream: Vec<u8>) -> Result<Chunk, QalamError> {
        let scanner = Scanner::new(stream);
        let mut chunk = Chunk::new();
        let mut compiler = Self {
            locals: Rc::new(RefCell::new(Vec::new())),
            local_count: 0,
            scope_depth: 0,
        };
        let parser = Parser::new(&scanner, &mut chunk, &mut compiler)?;
        parser.parse()?;
        return Ok(chunk);
    }

    pub fn add_local(&mut self, name: String, immutable: bool) {
        (*self.locals)
            .borrow_mut()
            .push(Local::new(name, self.scope_depth, false, immutable));
        self.local_count += 1;
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn resolve_local(&self, name: String, line: usize) -> Result<(Scope, bool), QalamError> {
        for i in (0..self.local_count).rev() {
            let local = &self.locals.borrow()[i];
            if local.name == name {
                if !local.init {
                    return Err(QalamError::with_line_compile(
                        "Can't read local variable in it's own initializer.",
                        line,
                    ));
                }
                return Ok((Scope::Local(i), local.immutable));
            }
        }
        return Ok((Scope::Global, false));
    }

    pub fn mark_initialized(&mut self) {
        self.locals.borrow_mut()[self.local_count - 1].depth = self.scope_depth;
        self.locals.borrow_mut()[self.local_count - 1].init = true;
    }

    pub fn end_scope<'b>(&mut self, chunk: &'b mut Chunk, line: usize) {
        self.scope_depth -= 1;
        let mut pop_count = 0;
        loop {
            if self.local_count == 0 {
                break;
            }

            if self.locals.borrow()[self.local_count - 1].depth <= self.scope_depth {
                break;
            }
            self.locals.borrow_mut().pop();
            self.local_count -= 1;
            pop_count += 1;
        }
        chunk.write(Box::new(PopN::new(pop_count)), line);
    }
}
