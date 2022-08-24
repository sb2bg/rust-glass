use logos::{Lexer, Logos};

fn lex_string(lex: &mut Lexer<Token>) -> String {
    let s = lex.slice();
    s[1..s.len() - 1].into()
}

#[derive(Logos, Debug)]
pub enum Token {
    //regex to match a number (integers and floats)
    #[regex(r"\d+(\.\d+)?", |lexer| lexer.slice().parse::<f64>().unwrap())]
    // regex to match byte numbers (integers and floats)
    Number(f64),

    // regex to match an identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lexer| lexer.slice().to_string())]
    Identifier(String),

    // regex to match a string
    #[regex(r#""(\\.|[^"])*""#, lex_string)]
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

    #[error]
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Error,
}
