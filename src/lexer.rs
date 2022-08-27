use logos::{Lexer, Logos};
use snailquote::unescape;

fn lex_string(lex: &mut Lexer<Token>) -> Option<String> {
    unescape(lex.slice()).ok()
}

fn lex_number_with_base(lex: &mut Lexer<Token>, radix: u32) -> Option<f64> {
    i64::from_str_radix(&lex.slice()[2..], radix)
        .ok()
        .map(|x| x as f64)
}

#[derive(Logos, Debug)]
pub enum Token {
    #[regex(r"\d+(\.\d+)?", |lexer| lexer.slice().parse::<f64>())] // decimal
    #[regex(r"0x[0-9A-Fa-f]+", |lexer| lex_number_with_base(lexer, 16))] // hexadecimal
    #[regex(r"0o[0-7]+", |lexer| lex_number_with_base(lexer, 8))] // octal
    #[regex(r"0b[01]+", |lexer| lex_number_with_base(lexer, 2))] // binary
    Number(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lexer| lexer.slice().to_string())] // identifier
    Identifier(String),

    #[regex(r#""(\\.|[^"])*""#, lex_string)] // string
    String(String),

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("**")]
    StarStar,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("=")]
    Equal,

    #[token("+=")]
    PlusEqual,

    #[token("-=")]
    MinusEqual,

    #[token("*=")]
    StarEqual,

    #[token("/=")]
    SlashEqual,

    #[token("%=")]
    PercentEqual,

    #[token("**=")]
    StarStarEqual,

    #[token("==")]
    EqualEqual,

    #[token("!=")]
    ExclamationEqual,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanEqual,

    #[token("not")]
    Not,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("return")]
    Return,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("import")]
    Import,

    #[token("match")]
    Match,

    #[token("print")]
    Print,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("void")]
    Void,

    #[token("func")]
    Func,

    #[token("=>")]
    Arrow,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("..")]
    DotDot,

    #[token("..=")]
    DotDotEqual,

    #[token("#")]
    Hash,

    #[error]
    #[regex(r"[ \t\n\r]+|//[^\n]*", logos::skip)] // skip whitespace and comments
    Error,
}
