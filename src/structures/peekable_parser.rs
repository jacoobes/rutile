use logos::{Lexer, SpannedIter, Span};
use super::tokens::Token;

pub struct Peekable<'source> {
    lexer: SpannedIter<'source ,Token>,
    peeked: Option<Option<(Token, Span)>>,
}

impl<'source> Peekable<'source> {
    pub fn new(source : Lexer<'source, Token>) -> Self {
        Self {
            lexer: source.spanned(),
            peeked: None,
        }
    }
    pub fn peek(&mut self) -> Option<&(Token, Span)> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        self.peeked.as_ref().unwrap().as_ref()
    }

    pub fn source(&self) {

    }
}

impl<'source> Iterator for Peekable<'source> {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }
}
