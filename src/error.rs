use crate::Token;
use git_version::git_version;
use line_span::find_line_range;
use logos::Span;
use std::fmt::Debug;
use std::rc::Rc;
use thiserror::Error;

fn get_token<'a>(source: &'a str, span: &'a Span) -> &'a str {
    source[span.start..span.end].trim()
}

fn get_line<'a>(source: &'a str, span: &'a Span) -> &'a str {
    &source[find_line_range(source, span.start)].trim()
}

// todo: instead of passing source to each error, just get the span and pass that to the error handler in main?
// I didn't think about if this is possible when I wrote this, but it might be ^
#[derive(Error, Debug)]
pub enum GlassError {
    #[error("Unknown error '{error_message}'. Please report this bug with the following information: Glass Version = '{}', Git Revision = '{}'", env!("CARGO_PKG_VERSION"), git_version!(fallback = "<unknown>"))]
    UnknownError { error_message: String },

    #[error(
        "Unknown token '{}' encountered at {}",
        get_token(src, span),
        get_line(src, span)
    )]
    UnknownToken { src: Rc<str>, span: Span },

    #[error("Unclosed string literal starting at {}", get_line(src, span))]
    UnclosedString { src: Rc<str>, span: Span },

    #[error(
        "Unknown escape sequence '{escape_sequence}' at {} {}",
        get_token(src, span),
        get_line(src, span)
    )]
    UnknownEscapeSequence {
        escape_sequence: String,
        src: Rc<str>,
        span: Span,
    },

    // this is a bit of a hack, but it's the best way I can think of to do this
    #[error(
        "{}",
        if let Some(expected) = expected {
            format!("Expected '{}' but found '{}' instead at {}", expected.get_rep(), get_token(src, span), get_line(src, span))
        } else {
            format!("Unexpected token '{}' at {}", get_token(src, span), get_line(src, span))
        }
    )]
    UnexpectedToken {
        expected: Option<Token>,
        src: Rc<str>,
        span: Span,
    },

    #[error("Unexpected end of input in source file '{filename}'")]
    UnexpectedEndOfInput { filename: Rc<str> },

    #[error("Cannot use operation '{operation}' on type '{left}' and '{right}'")]
    InvalidOperation {
        operation: String,
        left: String,
        right: String,
    },

    #[error("Unary operator '{operation}' cannot be applied to type '{operand}'")]
    InvalidUnaryOperation { operation: String, operand: String },

    // this error is to only be used in development as a placeholder for errors that haven't been implemented yet
    #[error("{message}")]
    PlaceholderError { message: String },
}
