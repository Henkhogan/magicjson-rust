use std::io::Read;
use std::fs;

mod wrapper;
use wrapper::{JsonBytesWrapper, JsonWrapperTrait};

pub mod objects;
use objects::{JsonItem, JsonKey};
pub use objects::JsonCustomType;


mod deserialize;
use deserialize::handle_dict_or_list;

mod serialize;

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

    use crate::serialize::{JsonSerializable};

    use self::serialize::ToJsonString;

    use super::*;
    use std::{any::Any, collections::{BTreeMap, HashMap}, fmt::Display};

    #[test]
    fn test_load_file() {
        // Call load_file function
        let result: JsonItem = load_file("tests/test0.json".to_string());

        // Check the result
        match result {
            JsonItem::Dict(dict) => {
                //let dict0 = dict.get("dict0").unwrap();
                assert_eq!(dict.len(), 1);
                //assert_eq!(dict.get("key").unwrap(), "value1");
            },
            _ => panic!("load_file failed"),
        }
    }

    #[test]
    fn test_serialize_jsonserializable() {
        // Create a HashMap
        let mut map: BTreeMap<String, JsonSerializable> = BTreeMap::new();
        map.insert("array".to_string(), JsonSerializable::Vec(vec![JsonSerializable::I32(1), JsonSerializable::I32(2)]));
        map.insert("bool".to_string(), JsonSerializable::Bool(true));
        map.insert("btreemap".to_string(), JsonSerializable::BTreeMap(BTreeMap::new()));
        map.insert("float".to_string(), JsonSerializable::F32(123.456));
        map.insert("hashmap".to_string(), JsonSerializable::HashMap(HashMap::new()));
        map.insert("int".to_string(), JsonSerializable::I32(123));
        map.insert("string".to_string(), JsonSerializable::String("value1".to_string()));
        map.insert("null".to_string(), JsonSerializable::Null);

        let result: JsonSerializable = map.into();
        let serialzed = result.to_string(); 

        // Check the result
        assert_eq!(serialzed, r#"{"array":[1,2],"bool":true,"btreemap":{},"float":123.456,"hashmap":{},"int":123,"null":null,"string":"value1"}"#.to_string());
    }

    #[test]
    fn test_serialize_stringhashmap() {
        // Create a HashMap
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        map.insert("string1key".to_string(), "string1value".to_string());
        map.insert("string2key".to_string(), "string2value".to_string());
        

        // Call serialize_any function
        let result: JsonSerializable = map.into();
        let serialzed = result.to_string();
        assert_eq!(serialzed, r#"{"string1key":"string1value","string2key":"string2value"}"#.to_string())
    }

   

    #[test]
    fn test_serialize_boxhashmap() {
        // Create a HashMap
        let mut map: BTreeMap<String, Box<dyn ToJsonString>> = BTreeMap::new();
        map.insert("array".to_string(), Box::new(vec![1,2]) as Box<Vec<i32>>);
        map.insert("bool".to_string(), Box::new(true));
        map.insert("btreemap".to_string(), Box::new(BTreeMap::<String, Box<dyn ToJsonString>>::new()) as Box<dyn ToJsonString>);
        map.insert("float".to_string(), Box::new(123.456));
        map.insert("hashmap".to_string(), Box::new(HashMap::<String, Box<dyn ToJsonString>>::new()) as Box<dyn ToJsonString>);
        map.insert("int".to_string(), Box::new(123));
        map.insert("null".to_string(), Box::new(None::<JsonSerializable>) as Box<dyn ToJsonString> );
        map.insert("string".to_string(), Box::new("value1".to_string()));


        // Call serialize_any function
        let result: JsonSerializable = map.to_json();
        let serialized = result.to_string(); 



        // Check the result
        assert_eq!(serialized, r#"{"array":[1,2],"bool":true,"btreemap":{},"float":123.456,"hashmap":{},"int":123,"null":null,"string":"value1"}"#.to_string());
    }//
}