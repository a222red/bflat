use logos::Span;

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub body: Box<Node>
}

#[derive(Debug)]
pub struct BinOp {
    l: Box<Node>,
    r: Box<Node>
}

impl BinOp {
    pub fn new(lhs: Node, rhs: Node) -> BinOp {
        BinOp { l: Box::new(lhs), r: Box::new(rhs) }
    }
}

#[derive(Debug)]
pub enum NodeType {
    FuncDec(Func),
    IntLiteral(i64),
    Add()
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    span: Span
}

impl Node {
    pub fn new(node_type: NodeType, span: Span) -> Node {
        Node { node_type, span }
    }
}
