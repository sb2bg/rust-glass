use std::collections::HashMap;
use std::ops::Add;

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

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        println!("add {:?} {:?}", self, other);
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
