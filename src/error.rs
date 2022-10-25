use crate::Token;
use line_span::find_line_range;
use logos::{Source, Span};
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

fn get_token<'a>(source: &'a String, span: &'a Span) -> &'a str {
    source[span.start..span.end].trim()
}

fn get_line<'a>(source: &'a String, span: &'a Span) -> &'a str {
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
    UnknownToken { src: String, span: Span },

    #[error("Unclosed string literal starting at {}", get_line(src, span))]
    UnclosedString { src: String, span: Span },

    #[error(
        "Unknown escape sequence '{}' at {} {}",
        escape_sequence,
        get_token(src, span),
        get_line(src, span)
    )]
    UnknownEscapeSequence {
        escape_sequence: String,
        src: String,
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
        src: String,
        span: Span,
    },
    
    #[error("Unexpected end of input at {}", get_line(src, span))]
    UnexpectedEndOfInput { src: String, span: Span },
}
