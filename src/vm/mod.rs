use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::error::QalamError;
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;
use table::Table;
pub mod table;

pub struct VM {
    stack: Rc<RefCell<Vec<Value>>>,
    call_frame: Rc<RefCell<Vec<String>>>,
    globals: Rc<RefCell<Table>>,
    ip: RefCell<usize>,
}

impl VM {
    pub fn new() -> Self {
        return Self {
            stack: Rc::new(RefCell::new(Vec::new())),
            call_frame: Rc::new(RefCell::new(Vec::new())),
            globals: Rc::new(RefCell::new(Table::new())),
            ip: RefCell::new(0),
        };
    }

    fn debug(&self, chunk: &mut Chunk) {
        // print!("          ");
        for value in self.stack.borrow().iter() {
            print!("[ ");
            print!("{}", value);
            print!(" ]");
        }
        print!("\n");
        println!("{}", self.globals.borrow());
        print!("{}\n", chunk.code[*self.ip.borrow()]);
    }

    pub fn interpret(&mut self, src: Vec<u8>) -> Result<(), QalamError> {
        let mut chunk = Compiler::compile(src)?;
        // return Ok(());
        self.run(&mut chunk)
    }

    pub fn run(&mut self, chunk: &mut Chunk) -> Result<(), QalamError> {
        let n = chunk.code.len();
        // temporarily adding this in manually, later on it should be a generic Function that has this already dealt with
        self.call_frame.borrow_mut().push(String::from("__main__"));
        let call_frame_size = self.call_frame.borrow().len();
        loop {
            if *self.ip.borrow() >= n {
                break;
            }
            self.debug(chunk);
            let inst = &chunk.code[*self.ip.borrow()];
            let line = &chunk.lines[*self.ip.borrow()];
            let offset = inst.eval(
                self.stack.clone(),
                self.call_frame.clone(),
                self.globals.clone(),
                *line,
            );
            match offset {
                Ok(offset) => {
                    if offset > 0 {
                        self.ip.replace(offset);
                    } else {
                        self.ip.replace_with(|&mut old| old + 1);
                    }
                    if self.call_frame.borrow().len() < call_frame_size {
                        // hit return statement
                        let val = self.stack.borrow_mut().pop();
                        if let Some(val) = val {
                            print!("{}\n", val);
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(());
    }
}
