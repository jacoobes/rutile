use std::ops::Index;

#[derive(Debug, Default)]
pub struct Stack<T> {
    my_stack : Vec<T>,
    max : usize
}

impl <T> Stack<T> {
    
   pub fn new(max : usize ) -> Stack<T> {
       Stack { my_stack : Vec::new(), max } 
   }

   pub fn default() -> Stack<T> {
       Stack { my_stack: Vec::new(), max : 256 }
   }
   pub fn get(&self, idx : usize) -> Option<&T> {
        self.my_stack.get(idx)
   }
   pub fn pop (&mut self) -> Option<T> {
       self.my_stack.pop()
    }
   pub fn empty (&self) -> bool {
       self.my_stack.is_empty()
    }
   pub fn push(&mut self, val : T) {
       if self.my_stack.len() == self.max {
           panic!("Stack overflow: reached {}",self.max)
       }
       self.my_stack.push(val)
   }
   pub fn top(&self) -> Option<&T> {
       self.my_stack.last()
   }
   pub fn len(&self) -> usize {
       self.my_stack.len()
   }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

}

impl <T> Index<usize> for Stack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.my_stack[index]
    }
}
