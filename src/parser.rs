use crate::token::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<Value>),
    Table(HashMap<String, Value>)
}

pub fn parse(lexed: Vec<Token>) -> HashMap<String, Value> {
    let mut resmap = HashMap::new();

    let mut lexed_iter = lexed.iter();

    while let Some(token) = lexed_iter.next() {
        match token {
            _ => {}
        }
    }

    resmap
}
