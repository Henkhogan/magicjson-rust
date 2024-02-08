use std::collections::{HashMap, BTreeMap};
use std::fmt::Write;
use std::ops::Deref;


//use crate::constants::{DIGIT_CHARS, WHITESPACE_CHARS, QUOTE_CHARS, DICT_START_CHAR, DICT_END_CHAR, LIST_START_CHAR, LIST_END_CHAR, COMMA_CHAR, DOT_CHAR, MINUS_CHAR, PLUS_CHAR, ESCAPE_CHAR, AFTER_NULL_CHARS, LOOP_MAX_ITERATIONS, MAX_ITEMS, NUMERIC_CHARS, DATETIME_ID, TIMESTAMP_ID};

use chrono::NaiveDateTime;
use num_traits::PrimInt;


fn quote_and_escape_string(s: String) -> String {
    let mut escaped_string = "\"".to_string();
    for c in s.chars() {
        if c == '"' {
            let _ = escaped_string.write_str("\\\"");
        } else if c == '\\' {
            let _ = escaped_string.write_str("\\\\");
        } else {
            let _ = escaped_string.write_char(c);
        }
    }
    let _ = escaped_string.write_str("\"");
    return escaped_string
}

pub enum JsonSerializable {
    Null,
    Bool(bool),
    F32(f32),
    F64(f64),
    ISize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    USize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    String(String),
    Vec(Vec<Self>),
    HashMap(HashMap<String, Self>),
    BTreeMap(BTreeMap<String, Self>),
    NaiveDateTime(NaiveDateTime),
}

pub trait ToJsonString {
    fn to_json(&self) -> JsonSerializable;
    fn to_json_string(self) -> String;
 
}

pub trait ToJsonFromInt {
    fn to_json(&self) -> JsonSerializable;
}

impl <T> ToJsonString for T where T: ToJsonFromInt {
    fn to_json(&self) -> JsonSerializable {
        self.to_json()
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
} 
// bool
impl From<bool> for JsonSerializable {
    fn from(value: bool) -> Self {
        JsonSerializable::Bool(value)
    }
}

impl ToJsonString for bool {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::Bool(*self)
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}

impl From<&bool> for JsonSerializable {
    fn from(value: &bool) -> Self {
        JsonSerializable::Bool(*value)
    }
}

impl ToJsonString for &bool {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::Bool(**self)
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}

// float
impl From<f32> for JsonSerializable {
    fn from(value: f32) -> Self {
        JsonSerializable::F32(value)
    }   
}

impl ToJsonFromInt for f32 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::F32(*self)
    }
}

impl From<f64> for JsonSerializable {
    fn from(value: f64) -> Self {
        JsonSerializable::F64(value)
    }
}

impl ToJsonFromInt for f64 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::F64(*self)
    }
}

// int
impl From<isize> for JsonSerializable {
    fn from(value: isize) -> Self {
        JsonSerializable::ISize(value)
    }
}

impl ToJsonFromInt for isize {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::ISize(*self)
    }
}

impl From<i8> for JsonSerializable {
    fn from(value: i8) -> Self {
        JsonSerializable::I8(value)
    }
}

impl ToJsonFromInt for i8 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::I8(*self)
    }
}

impl From<i16> for JsonSerializable {
    fn from(value: i16) -> Self {
        JsonSerializable::I16(value)
    }
}

impl ToJsonFromInt for i16 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::I16(*self)
    }
}

impl From<i32> for JsonSerializable {
    fn from(value: i32) -> Self {
        JsonSerializable::I32(value)
    }
}

impl ToJsonFromInt for i32 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::I32(*self)
    }
}

impl From<i64> for JsonSerializable {
    fn from(value: i64) -> Self {
        JsonSerializable::I64(value)
    }
}

impl ToJsonFromInt for i64 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::I64(*self)
    }
}

impl From<i128> for JsonSerializable {
    fn from(value: i128) -> Self {
        JsonSerializable::I128(value)
    }
}

impl ToJsonFromInt for i128 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::I128(*self)
    }
}

// uint
impl From<usize> for JsonSerializable {
    fn from(value: usize) -> Self {
        JsonSerializable::USize(value)
    }
}

impl ToJsonFromInt for usize {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::USize(*self)
    }
}


impl From<u8> for JsonSerializable {
    fn from(value: u8) -> Self {
        JsonSerializable::U8(value)
    }
}

impl ToJsonFromInt for u8 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::U8(*self)
    }    
}

impl From<u16> for JsonSerializable {
    fn from(value: u16) -> Self {
        JsonSerializable::U16(value)
    }
}

impl ToJsonFromInt for u16 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::U16(*self)
    }
}

impl From<u32> for JsonSerializable {
    fn from(value: u32) -> Self {
        JsonSerializable::U32(value)
    }
}

impl ToJsonFromInt for u32 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::U32(*self)
    }
}


impl From<u64> for JsonSerializable {
    fn from(value: u64) -> Self {
        JsonSerializable::U64(value)
    }
}

impl ToJsonFromInt for u64 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::U64(*self)
    }
}

impl From<u128> for JsonSerializable {
    fn from(value: u128) -> Self {
        JsonSerializable::U128(value)
    }
}

impl ToJsonFromInt for u128 {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::U128(*self)
    }
}

impl From<&str> for JsonSerializable {
    fn from(value: &str) -> Self {
        JsonSerializable::String(value.to_string())
    }
}

impl ToJsonString for &str {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::String((*self).to_string())
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}

impl From<String> for JsonSerializable {
    fn from(value: String) -> Self {
        JsonSerializable::String(value)
    }
}

impl ToJsonString for String {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::String(self.to_string())
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}

impl From<Option<JsonSerializable>> for JsonSerializable {
    fn from(value: Option<JsonSerializable>) -> Self {
        match value {
            Some(v) => v,
            None => JsonSerializable::Null
        }
    }
}

impl ToJsonString for Option<JsonSerializable> {
    fn to_json(&self) -> JsonSerializable {
        match self {
            Some(v) => {
                match v {
                    JsonSerializable::Null => JsonSerializable::Null,
                    JsonSerializable::Bool(b) => JsonSerializable::Bool(*b),
                    JsonSerializable::F32(f) => JsonSerializable::F32(*f),
                    JsonSerializable::F64(f) => JsonSerializable::F64(*f),
                    JsonSerializable::ISize(i) => JsonSerializable::ISize(*i),
                    JsonSerializable::I8(i) => JsonSerializable::I8(*i),
                    JsonSerializable::I16(i) => JsonSerializable::I16(*i),
                    JsonSerializable::I32(i) => JsonSerializable::I32(*i),
                    JsonSerializable::I64(i) => JsonSerializable::I64(*i),
                    JsonSerializable::I128(i) => JsonSerializable::I128(*i),
                    JsonSerializable::USize(u) => JsonSerializable::USize(*u),
                    JsonSerializable::U8(u) => JsonSerializable::U8(*u),
                    JsonSerializable::U16(u) => JsonSerializable::U16(*u),
                    JsonSerializable::U32(u) => JsonSerializable::U32(*u),
                    JsonSerializable::U64(u) => JsonSerializable::U64(*u),
                    JsonSerializable::U128(u) => JsonSerializable::U128(*u),
                    JsonSerializable::String(s) => JsonSerializable::String(s.clone()),
                    //JsonSerializable::Vec(v) => JsonSerializable::Vec(*v),
                    //JsonSerializable::HashMap(h) => JsonSerializable::HashMap(*h),
                    //JsonSerializable::BTreeMap(b) => JsonSerializable::BTreeMap(*b),
                    JsonSerializable::NaiveDateTime(d) => JsonSerializable::NaiveDateTime(*d),
                    
                    _ => panic!("conversion failed")
                    
                }
            },
            None => JsonSerializable::Null
        }
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
    
}

impl <T> From<Vec<T>> for JsonSerializable
where T: Into<JsonSerializable> {
    fn from(value: Vec<T>) -> Self {
        JsonSerializable::Vec(value.into_iter().map(|v| v.into()).collect())
    }
}

impl <T> From<HashMap<String, T>> for JsonSerializable
where T: Into<JsonSerializable> {
    fn from(value: HashMap<String, T>) -> Self {
        JsonSerializable::HashMap(value.into_iter().map(|(k,v)| (k, v.into())).collect())
    }
}

impl <T> From<BTreeMap<String, T>> for JsonSerializable
where T: Into<JsonSerializable> {
    fn from(value: BTreeMap<String, T>) -> Self {
        JsonSerializable::BTreeMap(value.into_iter().map(|(k,v)| (k, v.into())).collect())
    }
}


impl From<NaiveDateTime> for JsonSerializable {
    fn from(value: NaiveDateTime) -> Self {
        JsonSerializable::NaiveDateTime(value)
    }
}

impl <T> ToJsonString for Vec<T>
where T: ToJsonFromInt {
    fn to_json(&self) -> JsonSerializable {
        JsonSerializable::Vec(self.iter().map(|v| v.to_json()).collect())
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}


impl ToJsonString for  BTreeMap<String, Box<dyn ToJsonString>> {
    fn to_json(&self) -> JsonSerializable {
        let mut map: BTreeMap<String, JsonSerializable> = BTreeMap::new();
        for (k,v) in self {
            map.insert(k.to_string(), v.to_json());
        }
        map.into()
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}

 impl ToJsonString for  HashMap<String, Box<dyn ToJsonString>> {
    fn to_json(&self) -> JsonSerializable {
        let mut map: BTreeMap<String, JsonSerializable> = BTreeMap::new();
        for (k,v) in self {
            map.insert(k.to_string(), v.to_json());
        }
        map.into()
    }
    fn to_json_string(self) -> String {
        self.to_json().to_string()
    }
}


impl ToString for JsonSerializable {
    fn to_string(&self) -> String {
        match self {
            JsonSerializable::Null => "null".to_string(),
            JsonSerializable::Bool(b) => b.to_string(),
            JsonSerializable::F32(f) => f.to_string(),
            JsonSerializable::F64(f) => f.to_string(),
            JsonSerializable::ISize(i) => i.to_string(),
            JsonSerializable::I8(i) => i.to_string(),
            JsonSerializable::I16(i) => i.to_string(),
            JsonSerializable::I32(i) => i.to_string(),
            JsonSerializable::I64(i) => i.to_string(),
            JsonSerializable::I128(i) => i.to_string(),
            JsonSerializable::USize(u) => u.to_string(),
            JsonSerializable::U8(u) => u.to_string(),
            JsonSerializable::U16(u) => u.to_string(),
            JsonSerializable::U32(u) => u.to_string(),
            JsonSerializable::U64(u) => u.to_string(),
            JsonSerializable::U128(u) => u.to_string(),
            JsonSerializable::String(s) => quote_and_escape_string(s.to_string()),
            JsonSerializable::Vec(a) => {
                let mut serialized_array = "[".to_string();
                let mut empty = true;
                for v in a {
                    let _ = serialized_array.write_fmt(format_args!("{},", v.to_string()));
                    empty = false;
                } 
                if !empty {
                    serialized_array.pop();
                }
                let _ = serialized_array.write_str("]");
                return serialized_array
            },
            JsonSerializable::HashMap(o) => {
                let mut serialized_hashmap = "{".to_string();
                let mut empty = true;
                for (k,v) in o {
                    let _ = serialized_hashmap.write_fmt(format_args!("\"{}\":{},", k, v.to_string()));
                    empty = false;
                }
                if !empty {
                    serialized_hashmap.pop();
                }
                let _ = serialized_hashmap.write_str("}");
                return serialized_hashmap
            },
            JsonSerializable::BTreeMap(o) => {
                let mut serialized_hashmap = "{".to_string();
                let mut empty = true;
                for (k,v) in o {
                    let _ = serialized_hashmap.write_fmt(format_args!("\"{}\":{},", k, v.to_string()));
                    empty = false;
                }
                if !empty {
                    serialized_hashmap.pop();
                }
                let _ = serialized_hashmap.write_str("}");
                return serialized_hashmap
            },
            //JsonSerializable::Object(o) => {
            //    let serialized_hashmap = "{ ".to_string();
            //    for (k,v) in o {
            //        serialized_hashmap.write_fmt(format!("\"{}\":{}, ", k, v.serialize()));
            //    }
            //    serialized_hashmap.write_str(" }");
            //    return serialized_hashmap
            //},
            JsonSerializable::NaiveDateTime(d) => d.to_string(),
        }
    }
}
