use crate::lexer::Token;
use std::collections::VecDeque;

use crate::error::GlassError;
use crate::node::Node;
use logos::Span;

pub struct Parser {
    tokens: VecDeque<(Token, Span)>,
    source: String,
    last_token: Option<(Token, Span)>,
    filename: String,
}

type ParseResult = Result<Node, GlassError>;

impl Parser {
    pub fn new(tokens: VecDeque<(Token, Span)>, source: String, filename: String) -> Self {
        Self {
            tokens,
            source,
            last_token: None,
            filename,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        loop {
            println!("{:?}", self.parse_atom()?);
        }
    }

    fn parse_expression(&mut self) -> ParseResult {
        todo!("parse_expression")
    }

    fn parse_math_expression(
        &mut self,
        a: &mut dyn FnMut() -> ParseResult,
        types: Vec<Token>,
    ) -> ParseResult {
        let mut left = a()?;

        while let Ok(Some((token, _))) = self.next() {
            if types.contains(&token) {
                let right = a()?;

                left = Node::BinaryOp {
                    left: Box::new(left),
                    op: token,
                    right: Box::new(right),
                };
            }
        }

        Ok(left)
    }

    fn parse_arithmetic(&mut self) -> ParseResult {
        self.parse_math_expression(&mut || self.parse_term(), vec![Token::Plus, Token::Minus])
    }

    fn parse_term(&mut self) -> ParseResult {
        self.parse_math_expression(
            &mut || self.parse_factor(),
            vec![Token::Star, Token::Slash, Token::Percent],
        )
    }

    fn parse_factor(&mut self) -> ParseResult {
        todo!("parse_factor")
    }

    fn parse_power(&mut self) -> ParseResult {
        todo!("parse_power")
    }

    fn parse_call(&mut self) -> ParseResult {
        todo!("parse_call")
    }

    fn parse_atom(&mut self) -> ParseResult {
        let token = self.next()?;

        match token {
            Some((Token::Number(num), _)) => Ok(Node::Number { value: num }),
            Some((Token::String(str), _)) => Ok(Node::String { value: str }),
            Some((Token::Identifier(ident), _)) => Ok(Node::Identifier { name: ident }),
            Some((Token::LParen, _)) => {
                let node = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(node)
            }
            _ => todo!("unimplemented token {:?}", token),
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), GlassError> {
        let next = self.next()?;

        if let Some((next_token, span)) = next {
            if next_token == token {
                Ok(())
            } else {
                Err(GlassError::UnexpectedToken {
                    expected: token,
                    actual: next_token,
                    src: self.source.clone(), // todo: don't clone
                    span,
                })
            }
        } else {
            Err(GlassError::UnexpectedEndOfInput {
                src: self.source.clone(),
                span: match &self.last_token {
                    Some((_, span)) => span.clone(),
                    None => {
                        return Err(GlassError::EmptyTokenStream {
                            src: self.source.clone(),
                            filename: self.filename.clone(), // todo: don't clone
                        });
                    }
                },
            })
        }
    }

    fn next(&mut self) -> Result<Option<(Token, Span)>, GlassError> {
        match self.tokens.pop_front() {
            Some((token, span)) => {
                match token {
                    Token::Error => {
                        return Err(GlassError::UnknownToken {
                            src: self.source.clone(), // todo: don't clone
                            span,
                        });
                    }
                    Token::UnclosedString => {
                        return Err(GlassError::UnclosedString {
                            src: self.source.clone(), // todo: don't clone
                            span,
                        });
                    }
                    Token::InvalidEscapeSequence => {
                        return Err(GlassError::UnknownEscapeSequence {
                            escape_sequence: self.source[span.start..span.end].to_string(),
                            src: self.source.clone(), // todo: don't clone
                            span,
                        });
                    }
                    _ => Ok(Some((token, span))),
                }
            }
            None => Ok(None),
        }
    }
}
