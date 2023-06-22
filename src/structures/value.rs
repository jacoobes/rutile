
use smol_str::SmolStr;

#[derive(Debug)]
pub enum Value {
    Str(SmolStr),
    Number(f64),
    Boolean(bool),
    Char(char),
    Ident(SmolStr)
}
