use std::collections::HashMap;
use chrono::{ NaiveDateTime, NaiveDate, NaiveTime, FixedOffset};

#[derive(Clone, Debug)]
pub enum Token {
    None,
    EOF,
    Newline,
    Comment,
    Eq,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Offset_DateTime(FixedOffset),
    Local_DateTime(NaiveDateTime),
    Local_Date(NaiveDate),
    Local_Time(NaiveTime),
    Array(Vec<Token>),
    Inline_Table(HashMap<String, Token>),
}
