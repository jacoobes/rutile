#[derive(Debug)]
pub struct Local {
    name: String,
    depth: usize
}
impl Local {
    pub fn less_than_self_depth(&self, dep : usize) -> bool {
        self.depth <= dep
    }
}

#[derive(Debug)]
pub struct LocalChart {
    locals : Vec<Local>,
    local_count: usize,
    scope_depth: usize
}
impl Default for LocalChart {
    fn default() -> Self {
        Self { locals: Vec::with_capacity(256), local_count : 0, scope_depth: 0 }
    }
}
impl LocalChart {

    pub fn inc_depth(&mut self) {
        self.scope_depth += 1;
    }

    pub fn dec_depth(&mut self) -> usize {
        self.scope_depth -= 1;
        let prev_len = self.locals.len();
        self.locals.retain(| el | el.less_than_self_depth(self.scope_depth + 1) );
        let now_len = self.locals.len();
        prev_len - now_len
    }

    pub fn new_local(&mut self, name: String) {
       if self.local_count == 256 {
            panic!("Variable overflow. Reached maximum variables per scope (256)")
       }
       let new_local = Local { name, depth: self.scope_depth };

       if self.locals_match(&new_local) {
            panic!("Already have another variable in the same scope!")
       }

       self.locals.push(new_local);
       self.local_count += 1;
    }
    pub fn locals_match(&self, new_local : &Local) -> bool {
        false
        //self.locals.iter().any( |t | t == new_local)
    }

    pub fn resolve_local(&self, name: String) -> Option<usize> {
        self.locals.iter().position(| s | s.name == name && s.depth == *self.cur_scope_depth())
    }

    pub fn cur_scope_depth(&self) -> &usize {
        &self.scope_depth
    }

    pub fn get(&self, idx : u8) -> Option<&Local> {
        self.locals.get(idx as usize)
    }

}

