use super::tokens::Token;

pub struct Local {
    name : Token,
    depth : usize
}
pub struct LocalChart {
    locals : Vec<Local>,
    local_count: usize,
    scope_depth: usize
}

impl LocalChart {

    pub fn new() -> Self {
        Self { locals: Vec::with_capacity(256), local_count : 0, scope_depth: 0 }
    }

}

impl Default for LocalChart {
    fn default() -> Self {
       Self::new()
    }
}
