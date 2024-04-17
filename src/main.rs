pub mod parser;
pub mod token;

use crate::token::*;
use crate::parser::*;

macro_rules! token_string {
    ($name:ident) => {
        Token::String(stringify!($name).to_string())
    };
    ($name:expr) => {
        Token::String(stringify!($name).to_string())
    };
}

fn main() {
    let test = vec![
        // Token::Value(String::from("test")),
        token_string!(test),
        Token::Eq,
        token_string!("45"),
        Token::EOF,
    ];

    dbg!(&test);

    println!("Hello, world!");
}
