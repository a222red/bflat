use crate::{
    lexer::{Lexer, Token},
    error::{error, error_here},
    ast::{Node, NodeType, Func}
};

type Lex<'source> = Lexer<'source, Token>;

macro_rules! next_tok {
    ($lex:expr) => {
        $lex.next().unwrap_or_else(|| error_here!($lex, "Unexpected EOF"))
    }
}

pub fn parse(lex: &mut Lex) -> Vec<Node> {
    let mut nodes = Vec::new();

    while let Some(tok) = lex.next() {
        nodes.push(match tok {
            Token::Fn => parse_function_dec(lex),
            _ => error_here!(lex, "Unexpected token")
        })
    }

    return nodes;
}

fn parse_function_dec(lex: &mut Lex) -> Node {
    let name = match next_tok!(lex) {
        Token::Name(n) => n,
        _ => error_here!(lex, "Expected function name after `fn`")
    };

    match next_tok!(lex) {
        Token::LParen => (),
        _ => error_here!(lex, "Expected `(` after function name")
    }

    match next_tok!(lex) {
        Token::RParen => (),
        _ => error_here!(lex, "Expected `)` after function name")
    }

    return Node::new(NodeType::FuncDec(Func {
        name,
        body: Box::new(parse_expr(lex))
    }), lex.span());
}

fn parse_expr(lex: &mut Lex) -> Node {
    match next_tok!(lex) {
        Token::IntLiteral(i) => Node::new(
            NodeType::IntLiteral(i),
            lex.span()
        ),
        _ => error_here!(lex, "Unexpected token")
    }
}
