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

    pub fn visit_bin_op_node(&self, op: &Token, left: &Box<Node>, right: &Box<Node>) -> Value {
        match op {
            Token::Plus => left.visit(self) + right.visit(self),
            Token::Minus => left.visit(self) - right.visit(self),
            Token::Star => left.visit(self) * right.visit(self),
            Token::Slash => left.visit(self) / right.visit(self),
            // Token::Percent => println!("{}", left.visit(self) % right.visit(self)),
            Token::StarStar => left.visit(self).pow(right.visit(self)),
            Token::EqualEqual => Value::Bool(left.visit(self) == right.visit(self)),
            Token::ExclamationEqual => Value::Bool(left.visit(self) != right.visit(self)),
            Token::LessThan => Value::Bool(left.visit(self) < right.visit(self)),
            Token::GreaterThan => Value::Bool(left.visit(self) > right.visit(self)),
            Token::LessThanEqual => Value::Bool(left.visit(self) <= right.visit(self)),
            Token::GreaterThanEqual => Value::Bool(left.visit(self) >= right.visit(self)),
            // Token::And => println!("{}", left.visit(self) && right.visit(self)),
            // Token::Or => println!("{}", left.visit(self) || right.visit(self)),
            _ => todo!("Error: Invalid binary operator"),
        }
    }

    pub fn visit_unary_op_node(&self, op: &Token, right: &Box<Node>) -> Value {
        match op {
            Token::Minus => -right.visit(self),
            Token::Not => !right.visit(self),
            Token::Plus => right.visit(self),
            _ => todo!("Error: Invalid unary operator"),
        }
    }

    pub fn visit_block_node(&self, statements: &Vec<Node>) -> Value {
        for statement in statements {
            statement.visit(self);
        }

        Value::Void
    }
}
