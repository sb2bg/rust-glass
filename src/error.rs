use crate::Token;
use line_span::find_line_range;
use logos::Span;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use thiserror::Error;

fn get_token<'a>(source: &'a str, span: &'a Span) -> &'a str {
    source[span.start..span.end].trim()
}

fn get_line<'a>(source: &'a str, span: &'a Span) -> &'a str {
    &source[find_line_range(source, span.start)].trim()
}

#[derive(Error, Debug)]
pub enum GlassError {
    #[error("Unknown error '{error_message}'. Please report this bug with the following information: Glass Version = '{glass_version}', Git Revision = '{git_revision}'")]
    UncaughtPanic {
        error_message: String,
        glass_version: String,
        git_revision: String,
    },

    #[error(
        "Unknown token '{}' encountered at {}",
        get_token(src, span),
        get_line(src, span)
    )]
    UnknownToken { src: Rc<str>, span: Span },

    #[error("Unclosed string literal starting at {}", get_line(src, span))]
    UnclosedString { src: Rc<str>, span: Span },

    #[error(
        "Unknown escape sequence '{}' at {} {}",
        escape_sequence,
        get_token(src, span),
        get_line(src, span)
    )]
    UnknownEscapeSequence {
        escape_sequence: String,
        src: Rc<str>,
        span: Span,
    },

    #[error(
        "Unexpected token '{}' at {}",
        get_token(src, span),
        get_line(src, span)
    )]
    UnexpectedToken {
        expected: Token,
        actual: Token,
        src: Rc<str>,
        span: Span,
    },

    #[error("Unexpected end of input in source file '{}'", filename)]
    UnexpectedEndOfInput { filename: Rc<str> },

    #[error("No parseable tokens found in source file '{}'", filename)]
    EmptyTokenStream { filename: Rc<str> },
}
