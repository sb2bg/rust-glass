use crate::lexer::Token;
use std::collections::VecDeque;

use crate::error::GlassError;
use crate::node::Node;
use logos::Span;

pub struct Parser {
    tokens: VecDeque<(Token, Span)>,
    source: String,
}

impl Parser {
    pub fn new(tokens: VecDeque<(Token, Span)>, source: String) -> Self {
        Self { tokens, source }
    }

    pub fn parse(&mut self) -> Result<(), GlassError> {
        while let Some(token) = self.next()? {}

        Ok(())
    }

    fn parse_expression(&mut self) -> Result<Node, GlassError> {
        todo!()
    }

    fn parse_atom(&mut self) -> Result<Node, GlassError> {
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
            _ => todo!(),
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
                    src: self.source.clone(),
                    span,
                })
            }
        } else {
            Err(GlassError::UnexpectedEndOfInput {
                src: self.source.clone(),
                span: todo!(),
            })
        }
    }

    fn next(&mut self) -> Result<Option<(Token, Span)>, GlassError> {
        match self.tokens.pop_front() {
            Some((token, span)) => {
                match token {
                    Token::Error => {
                        return Err(GlassError::UnknownToken {
                            src: self.source.clone(), // todo - don't clone
                            span,
                        });
                    }
                    Token::UnclosedString => {
                        return Err(GlassError::UnclosedString {
                            src: self.source.clone(), // todo - don't clone
                            span,
                        });
                    }
                    Token::InvalidEscapeSequence => {
                        return Err(GlassError::UnknownEscapeSequence {
                            escape_sequence: self.source[span.start..span.end].to_string(),
                            src: self.source.clone(), // todo - don't clone
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
