use crate::Token;
use logos::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a, Token>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> () {
        println!("FROM PARSER: {:?}", self.lexer.next());
    }
}
