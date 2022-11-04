use crate::lexer::Token;
use crate::node::Node;
use crate::value::Value;

pub struct Interpreter {
    context: (), // todo: implement context
}

impl Interpreter {
    pub fn new() -> Self {
        Self { context: () }
    }

    pub fn visit_node(&self, node: Node) -> Value {
        node.visit(self)
    }

    pub fn visit_number_node(&self, value: &f64) -> Value {
        Value::Num(*value)
    }

    pub fn visit_bin_op_node(&self, op: &Token, left: &Box<Node>, right: &Box<Node>) -> Value {
        match op {
            Token::Plus => left.visit(self) + right.visit(self),
            // Token::Minus => println!("{}", left.visit(self) - right.visit(self)),
            // Token::Star => println!("{}", left.visit(self) * right.visit(self)),
            // Token::Slash => println!("{}", left.visit(self) / right.visit(self)),
            // Token::Percent => println!("{}", left.visit(self) % right.visit(self)),
            // Token::StarStar => println!("{}", left.visit(self).powf(right.visit(self))),
            // Token::EqualEqual => println!("{}", left.visit(self) == right.visit(self)),
            // Token::ExclamationEqual => println!("{}", left.visit(self) != right.visit(self)),
            // Token::LessThan => println!("{}", left.visit(self) < right.visit(self)),
            // Token::GreaterThan => println!("{}", left.visit(self) > right.visit(self)),
            // Token::LessThanEqual => println!("{}", left.visit(self) <= right.visit(self)),
            // Token::GreaterThanEqual => println!("{}", left.visit(self) >= right.visit(self)),
            // Token::And => println!("{}", left.visit(self) && right.visit(self)),
            // Token::Or => println!("{}", left.visit(self) || right.visit(self)),
            _ => todo!("Error: Invalid binary operator"),
        }
    }
}
