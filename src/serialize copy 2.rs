use std::collections::{HashMap, BTreeMap};
use std::fmt::Write;
use std::ops::Deref;


//use crate::constants::{DIGIT_CHARS, WHITESPACE_CHARS, QUOTE_CHARS, DICT_START_CHAR, DICT_END_CHAR, LIST_START_CHAR, LIST_END_CHAR, COMMA_CHAR, DOT_CHAR, MINUS_CHAR, PLUS_CHAR, ESCAPE_CHAR, AFTER_NULL_CHARS, LOOP_MAX_ITERATIONS, MAX_ITEMS, NUMERIC_CHARS, DATETIME_ID, TIMESTAMP_ID};

use chrono::NaiveDateTime;

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
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
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

pub trait IntoJsonSerializable {
    fn into(self) -> JsonSerializable;
}

pub trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl ToJsonString for JsonSerializable {
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}

impl ToJsonString for BTreeMap<String, Box<dyn IntoJsonSerializable>> {
    fn to_json_string(&self) -> String {
        let mut serialized_hashmap = "{".to_string();
        let mut empty = true;
        for (k,v) in self {
            let _ = serialized_hashmap.write_fmt(format_args!("\"{}\":{},", k, v.to_json_string()));
            empty = false;
        }
        if !empty {
            serialized_hashmap.pop();
        }
        let _ = serialized_hashmap.write_str("}");
        return serialized_hashmap
    }
}

impl IntoJsonSerializable for bool {
    fn into(self) -> JsonSerializable {
        JsonSerializable::Bool(self)
    }
}



impl IntoJsonSerializable for f32 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::F32(self)
    }
}


impl IntoJsonSerializable for f64 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::F64(self)
    }
}

impl IntoJsonSerializable for i8 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I8(self)
    }
}

impl IntoJsonSerializable for i16 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I16(self)
    }
}

impl IntoJsonSerializable for i32 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I32(self)
    }
}

impl IntoJsonSerializable for i64 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I64(self)
    }
}

impl IntoJsonSerializable for i128 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I128(self)
    }
}

impl IntoJsonSerializable for u8 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U8(self)
    }
}

impl IntoJsonSerializable for u16 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U16(self)
    }
}

impl IntoJsonSerializable for u32 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U32(self)
    }
}

impl IntoJsonSerializable for u64 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U64(self)
    }
}

impl IntoJsonSerializable for u128 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U128(self)
    }
}

impl IntoJsonSerializable for String {
    fn into(self) -> JsonSerializable {
        JsonSerializable::String(self)
    }
}

impl IntoJsonSerializable for NaiveDateTime {
    fn into(self) -> JsonSerializable {
        JsonSerializable::NaiveDateTime(self)
    }
}

impl <T> IntoJsonSerializable for Vec<T>
where T: IntoJsonSerializable {
    fn into(self) -> JsonSerializable {
        JsonSerializable::Vec(self.into_iter().map(|v| v.into()).collect())
    }
}

impl <T> IntoJsonSerializable for BTreeMap<String, T>
where T: IntoJsonSerializable {
    fn into(self) -> JsonSerializable {
        JsonSerializable::BTreeMap(self.into_iter().map(|(k,v)| (k, v.into())).collect())
    }
}


impl <T> IntoJsonSerializable for HashMap<String, T>
where T: IntoJsonSerializable {
    fn into(self) -> JsonSerializable {
        JsonSerializable::HashMap(self.into_iter().map(|(k,v)| (k, v.into())).collect())
    }
}

impl <T> IntoJsonSerializable for Option<T>
where T: IntoJsonSerializable {
    fn into(self) -> JsonSerializable {
        match self {
            Some(o) => {
                o.into()
            }
            _ => {
                JsonSerializable::Null
            }
        }
    }
}

impl <T> IntoJsonSerializable for Box<T>
where T: IntoJsonSerializable {
    fn into(self) -> JsonSerializable {
        (*self).into()
    }
    
}




impl ToString for JsonSerializable {
    fn to_string(&self) -> String {
        match self {
            JsonSerializable::Null => "null".to_string(),
            JsonSerializable::Bool(b) => b.to_string(),
            JsonSerializable::F32(f) => f.to_string(),
            JsonSerializable::F64(f) => f.to_string(),
            JsonSerializable::I8(i) => i.to_string(),
            JsonSerializable::I16(i) => i.to_string(),
            JsonSerializable::I32(i) => i.to_string(),
            JsonSerializable::I64(i) => i.to_string(),
            JsonSerializable::I128(i) => i.to_string(),
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
