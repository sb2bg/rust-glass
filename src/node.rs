use crate::Token;

#[derive(Debug)]
pub enum Node {
    String {
        value: String,
    },
    Number {
        value: f64,
    },
    Identifier {
        name: String,
    },
    BinaryOp {
        op: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
    Assignment {
        op: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
    UnaryOp {
        op: Token,
        expr: Box<Node>,
    },
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },
    FunctionDefinition {
        name: String,
        args: Vec<String>,
        body: Box<Node>,
    },
    Return {
        value: Box<Node>,
    },
    If {
        condition: Box<Node>,
        body: Box<Node>,
        else_body: Option<Box<Node>>,
    },
    While {
        condition: Box<Node>,
        body: Box<Node>,
    },
    For {
        variable: String,
        start: Box<Node>,
        end: Box<Node>,
        step: Option<Box<Node>>,
        body: Box<Node>,
    },
    Block {
        statements: Vec<Node>,
    },
}
