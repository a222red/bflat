pub use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,
    #[token("%")]
    Percent,
    #[token("=")]
    Equals,
    #[token("==")]
    DoubleEquals,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("|")]
    BitwiseOr,
    #[token("&")]
    BitwiseAnd,
    #[token("^")]
    BitwiseXor,
    #[token("~")]
    BitwiseNot,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token("not")]
    Not,
    #[token("or")]
    Or,
    #[token("and")]
    And,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Name(String),
    #[regex("[-]?[0-9]+", |lex| lex.slice().parse())]
    #[regex("0x[0-9a-fA-f]+", |lex| i64::from_str_radix(&lex.slice()[2..], 16))]
    IntLiteral(i64),
    #[regex(
        "\"[^\"]*\"",
        |lex| {
            let s = lex.slice();
            s[1..s.len() - 1].to_owned()
        }
    )]
    String(String),
    #[regex("//.*", logos::skip)]
    LineComment,
    /// Ignored unless used for error handling
    #[error]
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Error,
}
