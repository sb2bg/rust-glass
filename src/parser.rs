use crate::lexer::Token;
use std::collections::VecDeque;

use logos::Span;

pub struct Parser {
    tokens: VecDeque<(Token, Span)>,
}

impl Parser {
    pub fn new(tokens: VecDeque<(Token, Span)>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        println!("FROM PARSER: {:?}", self.next());

        Ok(())
    }

    fn next(&mut self) -> Option<(Token, Span)> {
        self.tokens.pop_front()
    }
}
