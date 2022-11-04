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
            Token::Plus => left.visit(self)?.add(right.visit(self)?),
            Token::Minus => left.visit(self)?.sub(right.visit(self)?),
            Token::Star => left.visit(self)?.mul(right.visit(self)?),
            Token::Slash => left.visit(self)?.div(right.visit(self)?),
            Token::Percent => left.visit(self)?.rem(right.visit(self)?),
            Token::StarStar => left.visit(self)?.pow(right.visit(self)?),
            Token::EqualEqual => left.visit(self)?.eq(right.visit(self)?),
            Token::ExclamationEqual => left.visit(self)?.ne(right.visit(self)?),
            Token::LessThan => left.visit(self)?.lt(right.visit(self)?),
            Token::GreaterThan => left.visit(self)?.gt(right.visit(self)?),
            Token::LessThanEqual => left.visit(self)?.le(right.visit(self)?),
            Token::GreaterThanEqual => left.visit(self)?.ge(right.visit(self)?),
            Token::And => left.visit(self)?.and(right.visit(self)?),
            Token::Or => left.visit(self)?.or(right.visit(self)?),
            _ => Err(GlassError::PlaceholderError {
                message: "unimplemented bin op".to_string(),
            }),
        }
    }

    pub fn visit_unary_op_node(&self, op: &Token, right: &Box<Node>) -> InterpreterResult {
        match op {
            Token::Minus => right.visit(self)?.neg(),
            Token::Not => right.visit(self)?.not(),
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
