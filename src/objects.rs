use std::collections::HashMap;
use chrono::NaiveDateTime;

pub type JsonKey = String;

#[derive(Clone, Debug)]
pub struct JsonCustomType {
    pub name: String,
    pub value: String,
}


#[derive(Clone, Debug)]
pub enum JsonItem {
    Bool(bool),
    Dict(HashMap<JsonKey, JsonItem>),
    Int(i32),
    List(Vec<JsonItem>),
    Float(f64),
    Null(), 
    Str(String),
    Custom(JsonCustomType),
    Datetime(NaiveDateTime),
    Timestamp(f64),
}


