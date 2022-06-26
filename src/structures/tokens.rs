
use logos::{Lexer, Logos};
use smol_str::SmolStr;

#[derive(Default)]
pub struct MetaData {
    pub line_breaks: usize,
}

/// Regular grammar for Nums
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = MetaData)]
#[logos(subpattern int = r"\d+")]

pub enum Token {
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("use")]
    Use,
    #[token("stop")]
    Stop,
    #[token("continue")]
    Continue,
    #[token("expose")]
    Expose,
    #[token("return")]
    Return,
    #[token("start")]
    Start,
    #[token("end")]
    End,
    #[token("<")]
    LeftArr,
    #[token(">")]
    RightArr,
    #[token("/")]
    FowardSlash,

    /// standard 4 byte numbers
    #[regex(r"(?&int)", parse_num)]
    #[regex(r"\d+(?:\.\d+)+", parse_dub)]
    #[regex(r"\d+(?:\.\d+)?%", percent_float)]
    Number(f64),
    ///only ascii!
    ///smolstr is heap allocated if 23 bytes + ofc
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, make_str)]
    String(SmolStr),
    #[regex(r"true|false", parse_bool)]
    Bool(bool),
    #[regex(r"[a-zA-Z][_0-9a-zA-Z]*",  callback = |lex| SmolStr::from(lex.slice()))]
    Identifier(SmolStr),
    #[token(",")]
    Comma,
    #[token("!")]
    Bang,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("?")]
    Question,
    #[token(";")]
    Semi,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("|")]
    Bar,
    #[token(":")]
    Colon,
    #[token("*")]
    Star,
    #[token("^")]
    Caret,
    #[token("[")]
    LeftBrack,
    #[token("]")]
    RightBrack,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("=")]
    Assign,
    #[token(">=")]
    GreaterEq,
    #[token("<=")]
    LessEq,
    //multiline comments :> (anything) <:
    #[regex(r":>[^<]*(?:[^<:]*)<:", logos::skip)]
    //single line comments
    #[regex(r"~~[^\n]*\n", logos::skip)]
    //skip
    #[token("\n", |lex| lex.extras.line_breaks += 1; logos::Skip)]
    #[regex(r"[ \t\f\r]", logos::skip)]
    #[error]
    Error,
}

fn parse_num(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    let n = slice.parse().ok()?;
    Some(n)
}

fn parse_dub(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    slice.parse().ok()
}

fn make_str(lex: &mut Lexer<Token>) -> Option<SmolStr> {
    let slice = lex.slice();
    Some(SmolStr::new(&slice[1..slice.len() - 1]))
}

fn percent_float(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    let n: f32 = slice[..slice.len() - 1].parse().ok()?;
    Some((n / 100.) as f64)
}
fn parse_bool(lex: &mut Lexer<Token>) -> Option<bool> {
    let slice = lex.slice();
    slice.parse().ok()
}
