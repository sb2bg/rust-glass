use crate::value::Value;
use std::collections::HashMap;

pub struct Context {
    name: String,
    parent: Option<Box<Context>>,
    variables: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: HashMap::new(), // todo add builtins
            name: "global".into(),
        }
    }

    pub fn new_child<T: Into<String>>(&self, name: T) -> Self {
        Self {
            parent: None,
            variables: HashMap::new(),
            name: name.into(),
        }
    }

    // pub fn get(&self, name: &str) -> Option<Value> {
    //     if let Some(&value) = self.variables.get(name) {
    //         Some(value)
    //     } else if let Some(parent) = &self.parent {
    //         parent.get(name)
    //     } else {
    //         None
    //     }
    // }
    //
    // // todo: variables set in a child context should update the ones in the parent context if they have the same name?
    // pub fn set(&mut self, name: &str, value: Value) {
    //     self.variables.insert(name.into(), value);
    // }
    //
    // pub fn stack_trace(&self) -> String {
    //     let mut stack = vec![self.name.clone()];
    //     let mut current = self;
    //
    //     while let Some(parent) = &current.parent {
    //         stack.push(parent.name.clone());
    //         current = parent;
    //     }
    //     stack.reverse();
    //     stack.join(" -> ")
    // }
}
