use crate::error::GlassError;
use crate::lexer::Token;
use crate::node::Node;
use crate::value::Value;

pub struct Interpreter {
    context: (), // todo: implement context
}

pub type InterpreterResult = Result<Value, GlassError>;

impl Interpreter {
    pub fn new() -> Self {
        Self { context: () }
    }

    pub fn visit_node(&self, node: Node) -> InterpreterResult {
        node.visit(self)
    }

    pub fn visit_bin_op_node(
        &self,
        op: &Token,
        left: &Box<Node>,
        right: &Box<Node>,
    ) -> InterpreterResult {
        match op {
            Token::Plus => Ok(left.visit(self)? + right.visit(self)?),
            Token::Minus => Ok(left.visit(self)? - right.visit(self)?),
            Token::Star => Ok(left.visit(self)? * right.visit(self)?),
            Token::Slash => Ok(left.visit(self)? / right.visit(self)?),
            // Token::Percent => println!("{}", left.visit(self) % right.visit(self)),
            Token::StarStar => Ok(left.visit(self)?.pow(right.visit(self)?)),
            Token::EqualEqual => Ok(Value::Bool(left.visit(self)? == right.visit(self)?)),
            Token::ExclamationEqual => Ok(Value::Bool(left.visit(self)? != right.visit(self)?)),
            Token::LessThan => Ok(Value::Bool(left.visit(self)? < right.visit(self)?)),
            Token::GreaterThan => Ok(Value::Bool(left.visit(self)? > right.visit(self)?)),
            Token::LessThanEqual => Ok(Value::Bool(left.visit(self)? <= right.visit(self)?)),
            Token::GreaterThanEqual => Ok(Value::Bool(left.visit(self)? >= right.visit(self)?)),
            Token::And => Ok(left.visit(self)?.and(right.visit(self)?)),
            Token::Or => Ok(left.visit(self)?.or(right.visit(self)?)),
            _ => Err(GlassError::PlaceholderError {
                message: "unimplemented bin op".to_string(),
            }),
        }
    }

    pub fn visit_unary_op_node(&self, op: &Token, right: &Box<Node>) -> InterpreterResult {
        match op {
            Token::Minus => Ok(-right.visit(self)?),
            Token::Not => Ok(!right.visit(self)?),
            Token::Plus => right.visit(self),
            _ => Err(GlassError::PlaceholderError {
                message: "Invalid unary operator".to_string(),
            }),
        }
    }

    pub fn visit_block_node(&self, statements: &Vec<Node>) -> InterpreterResult {
        for statement in statements {
            statement.visit(self);
        }

        Ok(Value::Void)
    }
}
