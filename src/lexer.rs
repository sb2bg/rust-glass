use logos::{Lexer, Logos};
use snailquote::unescape;

fn lex_string(lex: &mut Lexer<Token>) -> Result<String, String> {
    match unescape(lex.slice()) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string().as_str()[15..17].into()), // todo: make this better
    }
}

fn lex_number_with_base(lex: &mut Lexer<Token>, radix: u32) -> Option<f64> {
    i64::from_str_radix(&lex.slice()[2..], radix)
        .ok()
        .map(|x| x as f64)
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"\d+(\.\d+)?", |lexer| lexer.slice().parse::<f64>())] // decimal
    #[regex(r"0x[0-9A-Fa-f]+", |lexer| lex_number_with_base(lexer, 16))] // hexadecimal
    #[regex(r"0o[0-7]+", |lexer| lex_number_with_base(lexer, 8))] // octal
    #[regex(r"0b[01]+", |lexer| lex_number_with_base(lexer, 2))] // binary
    Number(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lexer| lexer.slice().to_string())] // identifier
    Identifier(String),

    #[regex(r#""(\\.|[^"])*""#, lex_string)] // string
    UnverifiedString(Result<String, String>),

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

    #[token("..")] // range
    DotDot,

    #[token("...")] // spread operator
    DotDotDot,

    #[token("..=")] // inclusive range
    DotDotEqual,

    #[token("#")]
    Hash,

    #[regex(r#""(\\.|[^"])*"#)] // unclosed string
    UnclosedString,

    #[error]
    #[regex(r"[ \t\n\r]+|//[^\n]*", logos::skip)]
    Error,
}

// todo: there has to be a better way to do this
impl Token {
    pub fn get_rep(&self) -> &str {
        match self {
            Token::Number(_) => "number",
            Token::Identifier(_) => "identifier",
            Token::String(_) => "string",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::StarStar => "**",
            Token::Slash => "/",
            Token::Percent => "%",
            Token::Equal => "=",
            Token::PlusEqual => "+=",
            Token::MinusEqual => "-=",
            Token::StarEqual => "*=",
            Token::SlashEqual => "/=",
            Token::PercentEqual => "%=",
            Token::StarStarEqual => "**=",
            Token::EqualEqual => "==",
            Token::ExclamationEqual => "!=",
            Token::LessThan => "<",
            Token::LessThanEqual => "<=",
            Token::GreaterThan => ">",
            Token::GreaterThanEqual => ">=",
            Token::Not => "not",
            Token::And => "and",
            Token::Or => "or",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::For => "for",
            Token::In => "in",
            Token::Return => "return",
            Token::Break => "break",
            Token::Continue => "continue",
            Token::Import => "import",
            Token::Match => "match",
            Token::True => "true",
            Token::False => "false",
            Token::Void => "void",
            Token::Func => "func",
            Token::Arrow => "=>",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::Semicolon => ";",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::DotDot => "..",
            Token::DotDotDot => "...",
            Token::DotDotEqual => "..=",
            Token::Hash => "#",
            Token::UnclosedString => "unclosed string",
            Token::UnverifiedString(_) => "unverified string",
            Token::Error => "error",
        }
    }
}
