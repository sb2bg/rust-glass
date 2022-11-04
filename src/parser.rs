use crate::lexer::Token;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::error::GlassError;
use crate::node::Node;
use logos::Span;

pub struct Parser {
    tokens: VecDeque<(Token, Span)>,
    src: Rc<str>,
    filename: Rc<str>,
}

type ParseResult = Result<Node, GlassError>;

macro_rules! token_matches {
    ($token:expr, $($pattern:pat_param)|+) => {
        match $token {
            $($pattern)|+ => true,
            _ => false,
        }
    };
}

impl Parser {
    pub fn new(tokens: VecDeque<(Token, Span)>, src: Rc<str>, filename: Rc<str>) -> Self {
        Self {
            tokens,
            src,
            filename,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParseResult {
        return self.parse_equality();
    }

    fn parse_math_expression(
        &mut self,
        mut a: Box<dyn FnMut(&mut Self) -> ParseResult>,
        mut b: Option<Box<dyn FnMut(&mut Self) -> ParseResult>>,
        types: Vec<Token>,
    ) -> ParseResult {
        let mut left = a(self)?;

        while let Ok(Some((token, _))) = self.peek() {
            if types.contains(&token) {
                self.next()?;

                let right = match b {
                    Some(ref mut b) => b(self)?,
                    None => a(self)?,
                };

                left = Node::BinaryOp {
                    left: Box::new(left),
                    op: token,
                    right: Box::new(right),
                };

                continue;
            }

            break;
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> ParseResult {
        self.parse_math_expression(
            Box::new(Self::parse_comparison),
            None,
            vec![Token::EqualEqual, Token::ExclamationEqual],
        )
    }

    fn parse_comparison(&mut self) -> ParseResult {
        self.parse_math_expression(
            Box::new(Self::parse_term),
            None,
            vec![
                Token::LessThan,
                Token::GreaterThan,
                Token::LessThanEqual,
                Token::GreaterThanEqual,
            ],
        )
    }

    fn parse_term(&mut self) -> ParseResult {
        self.parse_math_expression(
            Box::new(Self::parse_factor),
            None,
            vec![Token::Plus, Token::Minus],
        )
    }

    fn parse_factor(&mut self) -> ParseResult {
        self.parse_math_expression(
            Box::new(Self::parse_power),
            None,
            vec![Token::Star, Token::Slash, Token::Percent],
        )
    }

    // note: not 100% sure this is correct, but I think it is
    fn parse_power(&mut self) -> ParseResult {
        self.parse_math_expression(Box::new(Self::parse_unary), None, vec![Token::StarStar])
    }

    fn parse_unary(&mut self) -> ParseResult {
        if let Ok(Some((token, _))) = self.peek() {
            if token_matches!(token, Token::Minus | Token::Plus | Token::Not) {
                self.next()?;

                return Ok(Node::UnaryOp {
                    op: token,
                    expr: Box::new(self.parse_unary()?),
                });
            }

            return self.parse_atom();
        }

        Err(GlassError::UnexpectedEndOfInput {
            filename: Rc::clone(&self.filename),
        })
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
            Some((_, span)) => Err(GlassError::UnexpectedToken {
                expected: None,
                src: Rc::clone(&self.src),
                span,
            }),
            None => Err(GlassError::UnexpectedEndOfInput {
                filename: Rc::clone(&self.filename),
            }),
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), GlassError> {
        let next = self.next()?;

        if let Some((next_token, span)) = next {
            if next_token == token {
                Ok(())
            } else {
                Err(GlassError::UnexpectedToken {
                    expected: Some(token),
                    src: Rc::clone(&self.src),
                    span,
                })
            }
        } else {
            Err(GlassError::UnexpectedEndOfInput {
                filename: Rc::clone(&self.filename),
            })
        }
    }

    fn next(&mut self) -> Result<Option<(Token, Span)>, GlassError> {
        Ok(if let Some((token, span)) = self.tokens.pop_front() {
            Some(self.check_error(token, span)?)
        } else {
            None
        })
    }

    fn peek(&mut self) -> Result<Option<(Token, Span)>, GlassError> {
        // todo: don't clone (more important than the other clones)
        Ok(if let Some((token, span)) = self.tokens.front().cloned() {
            Some(self.check_error(token, span)?)
        } else {
            None
        })
    }

    fn check_error(&mut self, token: Token, span: Span) -> Result<(Token, Span), GlassError> {
        match token {
            Token::Error => {
                return Err(GlassError::UnknownToken {
                    src: Rc::clone(&self.src),
                    span,
                });
            }
            Token::UnclosedString => {
                return Err(GlassError::UnclosedString {
                    src: Rc::clone(&self.src),
                    span,
                });
            }
            Token::InvalidEscapeSequence => {
                return Err(GlassError::UnknownEscapeSequence {
                    escape_sequence: self.src[span.start..span.end].to_string(),
                    src: Rc::clone(&self.src),
                    span,
                });
            }
            _ => Ok((token, span)),
        }
    }
}
