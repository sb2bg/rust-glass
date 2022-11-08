use crate::Token;
use git_version::git_version;
use line_span::{find_line_end, find_line_start};
use logos::Span;
use std::cmp::min;
use std::fmt::Debug;
use std::rc::Rc;
use thiserror::Error;

fn get_token<'a>(src: &'a str, span: &'a Span) -> &'a str {
    src[span.start..span.end].trim()
}

fn get_line<'a>(src: &'a str, filename: &'a str, span: &'a Span) -> String {
    let start = find_line_start(src, span.start);
    let end = find_line_end(src, span.start);
    let line = &src[start..end].trim();
    let line_num = src[..span.start].lines().count();
    let affected_range = span.start - start..span.end - start;

    format!(
        "\n\n\t{line}\n\t{}{}\n[{filename}(Ln:{line_num}, Col:{affected_range:?})]",
        &" ".repeat(affected_range.start),
        &"^".repeat(min(affected_range.len(), line.len() - affected_range.start)),
    )
}

// todo: instead of passing source to each error, just get the span and pass that to the error handler in main?
// I didn't think about if this is possible when I wrote this, but it might be ^
#[derive(Error, Debug)]
pub enum GlassError {
    #[error("Unknown error '{error_message}'. Please report this bug with the following information: Glass Version = '{}', Git Revision = '{}'", env!("CARGO_PKG_VERSION"), git_version!(fallback = "<unknown>"))]
    UnknownError { error_message: String },

    #[error("File '{filename}' not found")]
    FileNotFound { filename: Rc<str> },

    #[error(
        "Unknown token '{}' encountered at {}",
        get_token(src, span),
        get_line(src, filename, span)
    )]
    UnknownToken {
        src: Rc<str>,
        span: Span,
        filename: Rc<str>,
    },

    #[error(
        "Unclosed string literal starting at {}",
        get_line(src, filename, span)
    )]
    UnclosedString {
        src: Rc<str>,
        span: Span,
        filename: Rc<str>,
    },

    #[error(
        "Unknown escape sequence '{escape_sequence}' at {}",
        get_line(src, filename, span)
    )]
    UnknownEscapeSequence {
        escape_sequence: String,
        src: Rc<str>,
        filename: Rc<str>,
        span: Span,
    },

    // this is a bit of a hack, but it's the best way I can think of to do this
    #[error(
        "{}",
        if let Some(expected) = expected {
            format!("Expected '{}' but found '{}' instead at {}", expected.get_rep(), get_token(src, span), get_line(src, filename, span))
        } else {
            format!("Unexpected token '{}' at {}", get_token(src, span), get_line(src, filename, span))
        }
    )]
    UnexpectedToken {
        expected: Option<Token>,
        src: Rc<str>,
        filename: Rc<str>,
        span: Span,
    },

    #[error("Unexpected end of file in source file '{filename}'")]
    UnexpectedEndOfInput { filename: Rc<str> },

    #[error(
        "Cannot use operation '{operation}' on type '{left}' and '{right}' at {}",
        "get_line(src, filename, span)"
    )]
    InvalidOperation {
        operation: String,
        left: String,
        right: String,
        // todo: once we get spans and sources passed to interpreter
        // src: Rc<str>,
        // filename: Rc<str>,
        // span: Span,
    },

    #[error("Unary operator '{operation}' cannot be applied to type '{operand}'")]
    InvalidUnaryOperation { operation: String, operand: String },

    // this error is to only be used in development as a placeholder for errors that haven't been implemented yet
    #[error("{message}")]
    PlaceholderError { message: String },
}
