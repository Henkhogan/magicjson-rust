use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::fmt::Write;
use std::iter::{Empty, Map};


use crate::constants::{DIGIT_CHARS, WHITESPACE_CHARS, QUOTE_CHARS, DICT_START_CHAR, DICT_END_CHAR, LIST_START_CHAR, LIST_END_CHAR, COMMA_CHAR, DOT_CHAR, MINUS_CHAR, PLUS_CHAR, ESCAPE_CHAR, AFTER_NULL_CHARS, LOOP_MAX_ITERATIONS, MAX_ITEMS, NUMERIC_CHARS, DATETIME_ID, TIMESTAMP_ID};

use crate::objects::{JsonItem, JsonKey, JsonCustomType};

use crate::wrapper::{JsonBytesWrapper, JsonWrapperTrait};

//use iso8601::{DateTime as IsoDateTime};
use chrono::NaiveDateTime;

fn quote_and_escape_string(s: String) -> String {
    let mut escaped_string = "\"".to_string();
    for c in s.chars() {
        if c == '"' {
            escaped_string.write_str("\\\"");
        } else if c == '\\' {
            escaped_string.write_str("\\\\");
        } else {
            escaped_string.write_char(c);
        }
    }
    escaped_string.write_str("\"");
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
    Sequence(Vec<Self>),
    Dict(HashMap<String, Self>),
    NaiveDateTime(NaiveDateTime),
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
            JsonSerializable::Sequence(a) => {
                let mut serialized_array = "[".to_string();
                let mut empty = true;
                for v in a {
                    serialized_array.write_fmt(format_args!("{}, ", v.to_string()));
                    serialized_array.write_str(", ");
                    empty = false;
                } 
                if !empty {
                    serialized_array.pop();
                    serialized_array.pop();
                }
                serialized_array.write_str("]");
                return serialized_array
            },
            JsonSerializable::Dict(o) => {
                let mut serialized_hashmap = "{".to_string();
                let mut empty = true;
                for (k,v) in o {
                    serialized_hashmap.write_fmt(format_args!("\"{}\":{}, ", k, v.to_string()));
                    empty = false;
                }
                if !empty {
                    serialized_hashmap.pop();
                    serialized_hashmap.pop();
                }
                serialized_hashmap.write_str("}");
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

impl Into<JsonSerializable> for bool {
    fn into(self) -> JsonSerializable {
        JsonSerializable::Bool(self)
    }
}

impl Into<JsonSerializable> for f32 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::F32(self)
    }
}

impl Into<JsonSerializable> for f64 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::F64(self)
    }
}

impl Into<JsonSerializable> for i8 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I8(self)
    }
}

impl Into<JsonSerializable> for i16 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I16(self)
    }
}

impl Into<JsonSerializable> for i32 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I32(self)
    }
}

impl Into<JsonSerializable> for i64 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I64(self)
    }
}

impl Into<JsonSerializable> for i128 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::I128(self)
    }
}

impl Into<JsonSerializable> for u8 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U8(self)
    }
}

impl Into<JsonSerializable> for u16 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U16(self)
    }
}

impl Into<JsonSerializable> for u32 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U32(self)
    }
}

impl Into<JsonSerializable> for u64 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U64(self)
    }
}

impl Into<JsonSerializable> for u128 {
    fn into(self) -> JsonSerializable {
        JsonSerializable::U128(self)
    }
}


impl Into<JsonSerializable> for String {
    fn into(self) -> JsonSerializable {
        JsonSerializable::String(self)
    }
}


impl <T> Into<JsonSerializable> for Vec<T> where T: Into<JsonSerializable> {
    fn into(self) -> JsonSerializable {
        return JsonSerializable::Sequence(self.into_iter().map(|v| v.into()).collect())
    }
}


impl <T> Into<JsonSerializable> for HashMap<String, T> where T: Into<JsonSerializable> {
    fn into(self) -> JsonSerializable {
        return JsonSerializable::Dict(self.into_iter().map(|(k,v)| (k, v.into())).collect());
    }
}


impl Into<JsonSerializable> for NaiveDateTime {
    fn into(self) -> JsonSerializable {
        JsonSerializable::NaiveDateTime(self)
    }
}
