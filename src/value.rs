use crate::error::GlassError;
use crate::interpreter::InterpreterResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Num(f64),
    Str(String),
    Bool(bool),
    // Func(Function), // todo
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    // Struct(Struct), // todo
    Void,
}

impl Value {
    fn get_type(&self) -> String {
        match self {
            Value::Num(_) => "number",
            Value::Str(_) => "string",
            Value::Bool(_) => "boolean",
            Value::List(_) => "list",
            Value::Dict(_) => "dictionary",
            Value::Void => "void",
        }
        .into()
    }

    pub fn pow(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a.powf(b))),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "**".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn and(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "and".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn or(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "or".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn add(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a + b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
            (Value::List(mut a), Value::List(b)) => {
                a.extend(b);
                Ok(Value::List(a))
            }
            (Value::Str(a), Value::Num(b)) => Ok(Value::Str(a + &b.to_string())),
            (Value::Num(a), Value::Str(b)) => Ok(Value::Str(a.to_string() + &b)),
            (Value::Dict(mut a), Value::Dict(b)) => {
                a.extend(b);
                Ok(Value::Dict(a))
            }
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "+".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn sub(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a - b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "-".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn mul(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a * b)),
            (Value::Str(a), Value::Num(b)) | (Value::Num(b), Value::Str(a)) => {
                Ok(Value::Str(a.repeat(b as usize)))
            }
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "*".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn div(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a / b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "/".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn rem(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Num(a % b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "%".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn eq(self, other: Value) -> InterpreterResult {
        Ok(Value::Bool(self == other))
    }

    pub fn ne(self, other: Value) -> InterpreterResult {
        Ok(Value::Bool(self != other))
    }

    pub fn lt(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Bool(a < b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "<".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn le(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Bool(a <= b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: "<=".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn gt(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Bool(a > b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: ">".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn ge(self, other: Value) -> InterpreterResult {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Ok(Value::Bool(a >= b)),
            (a, b) => Err(GlassError::InvalidOperation {
                operation: ">=".into(),
                left: a.get_type(),
                right: b.get_type(),
            }),
        }
    }

    pub fn not(self) -> InterpreterResult {
        match self {
            Value::Bool(a) => Ok(Value::Bool(!a)),
            a => Err(GlassError::InvalidUnaryOperation {
                operation: "!".into(),
                operand: a.get_type(),
            }),
        }
    }

    pub fn neg(self) -> InterpreterResult {
        match self {
            Value::Num(a) => Ok(Value::Num(-a)),
            a => Err(GlassError::InvalidUnaryOperation {
                operation: "-".into(),
                operand: a.get_type(),
            }),
        }
    }
}
