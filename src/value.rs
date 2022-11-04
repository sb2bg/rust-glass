use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug)]
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
    pub fn pow(self, other: Value) -> Self {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a.powf(b)),
            _ => todo!("Error: Invalid operands for exponentiation"),
        }
    }

    pub fn and(self, other: Value) -> Self {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
            _ => todo!("Error: Invalid operands for logical AND"),
        }
    }

    pub fn or(self, other: Value) -> Self {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
            _ => todo!("Error: Invalid operands for logical OR"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
            (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
            (Value::List(mut a), Value::List(b)) => {
                a.extend(b);
                Value::List(a)
            }
            (Value::Dict(mut a), Value::Dict(b)) => {
                a.extend(b);
                Value::Dict(a)
            }
            _ => todo!("not done adding all addable types"),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a - b),
            _ => todo!("not done subtracting all subtractable types"),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a * b),
            (Value::Str(a), Value::Num(b)) | (Value::Num(b), Value::Str(a)) => {
                Value::Str(a.repeat(b as usize))
            }
            _ => todo!("not done multiplying all multiplyable types"),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a / b),
            _ => todo!("not done dividing all dividable types"),
        }
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Dict(a), Value::Dict(b)) => a == b,
            (Value::Void, Value::Void) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a.partial_cmp(b),
            (a, b) => todo!(
                "Error: Invalid operands for comparison: {:?} and {:?}",
                a,
                b
            ),
        }
    }
}

impl Not for Value {
    type Output = Value;

    fn not(self) -> Value {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            _ => todo!("not done adding all notable types"),
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Num(n) => Value::Num(-n),
            _ => todo!("not done adding all negable types"),
        }
    }
}
