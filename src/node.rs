use crate::interpreter::Interpreter;
use crate::value::Value;
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

impl Node {
    pub fn visit(&self, interpreter: &Interpreter) -> Value {
        match self {
            Node::String { .. } => todo!("string"),
            Node::Number { value } => interpreter.visit_number_node(value),
            Node::Identifier { .. } => todo!("identifier"),
            Node::BinaryOp { op, left, right } => interpreter.visit_bin_op_node(op, left, right),

            Node::Assignment { .. } => todo!("assignment"),
            Node::UnaryOp { op, expr } => todo!("unary op"),
            Node::FunctionCall { .. } => todo!("function call"),
            Node::FunctionDefinition { .. } => todo!("function definition"),
            Node::Return { .. } => todo!("return"),
            Node::If { .. } => todo!("if"),
            Node::While { .. } => todo!("while"),
            Node::For { .. } => todo!("for"),
            Node::Block { .. } => todo!("block"),
        }
    }
}
