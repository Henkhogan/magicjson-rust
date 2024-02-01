use std::{collections::HashMap, io::Read};
use std::fs;

mod wrapper;
use wrapper::{JsonBytesWrapper, JsonWrapperTrait};

pub mod objects;
use objects::{JsonItem, JsonKey, JsonCustomType};

mod handler;
use handler::handle_dict_or_list;

mod serializer;

mod constants;

#[macro_use]
extern crate lazy_static;


/// Reads a JSON file and returns a JsonItem
pub fn load_file<T>(file_path: String) -> T where T: From<JsonItem> {

    let mut bufreader = std::io::BufReader::new(fs::File::open(&file_path).unwrap()).bytes();   
    
    let mut json_wrapper = JsonBytesWrapper {
        // Source: https://dev.to/oliverjumpertz/how-to-read-files-in-rust-525d?comments_sort=top
        current: bufreader.next().unwrap().unwrap(),
        bufreader: bufreader,
        index: 0,
        end_reached: false,
    };

    // Skip forward the first non-whitespace character
    json_wrapper.skip_whitespace();    
    return handle_dict_or_list(&mut json_wrapper);
}




/////////////////////////////////////////////////////////////////////////////////////
/// Tests
/// 
/// Run tests with `cargo test -- --nocapture`

#[cfg(test)]#[cfg(test)]
mod tests {
    use crate::serializer::JsonSerializable;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_load_file() {
        // Call load_file function
        let result: JsonItem = load_file("tests/test0.json".to_string());

        // Check the result
        match result {
            JsonItem::Dict(dict) => {
                assert_eq!(dict.len(), 1);
                //assert_eq!(dict.get("key").unwrap(), "value1");
            },
            _ => panic!("load_file failed"),
        }
    }

    #[test]
    fn test_serialize_any() {
        // Create a HashMap
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value1".to_string());

        // Call serialize_any function
        let result: JsonSerializable = map.into();

        // Check the result
        assert_eq!(result.to_string(), r#"{"key":"value1"}"#);
    }
}