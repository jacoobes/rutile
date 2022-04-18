
#[derive(Debug, Default)]
pub struct Stack<T> {
    my_stack : Vec<T>,
    max : isize
}

impl <T> Stack<T> {
    
   pub fn new(max : isize ) -> Stack<T> {
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
       self.my_stack.push(val)
   }
   pub fn top(&self) -> Option<&T> {
       self.my_stack.last()
   }
   

}
