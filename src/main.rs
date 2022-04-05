mod lexer;
mod error;
mod parser;
mod ast;
mod codegen;

use crate::{
    lexer::{Token, Logos},
    parser::parse
};

use std::fs::read_to_string;

fn main() {
    let raw = read_to_string("test.bflat").unwrap();
    let mut lex = Token::lexer(&raw);

    println!("{:#?}", parse(&mut lex));
}
