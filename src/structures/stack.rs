

struct Stack<T> {

    my_stack : Vec<T>

}

impl <T> Stack<T> {

   pub fn pop (&mut self) -> Option<T> {
        self.my_stack.pop()
    }
   pub fn empty (&self) -> bool {
        self.my_stack.is_empty()
    }
   pub fn push (&mut self, val : T) {
        self.my_stack.push(val);
   }
   pub fn peek(&self) -> Option<&T> {
       self.my_stack.last()
   }

}
