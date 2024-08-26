use std::rc::Rc;
use std::cell::RefCell;
use crate::value::Value;
use crate::chunk::Chunk;

pub struct VM {
  stack: Rc<RefCell<Vec<Value>>>,
  call_frame: Rc<RefCell<Vec<String>>>,
  ip: RefCell<usize>
}

impl VM {
  pub fn new() -> Self {
    return Self {
      stack: Rc::new(RefCell::new(Vec::new())),
      call_frame: Rc::new(RefCell::new(Vec::new())),
      ip: RefCell::new(0)
    }
  }

  fn debug(&self, chunk: &mut Chunk) {
    // print!("          ");
    for value in self.stack.borrow().iter() {
      print!("[ ");
      print!("{}", value);
      print!(" ]");
    }
    print!("\n");
    print!("{}\n", chunk.code[*self.ip.borrow()]);
  }

  pub fn interpret(&mut self, chunk: &mut Chunk) {
    self.run(chunk);
  }

  pub fn run(&mut self, chunk: &mut Chunk) {
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
      let offset = inst.eval(self.stack.clone(), self.call_frame.clone());
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
        },
        Err(e) => {
          println!("{}", e);
        }
      }
    }
  }
}