use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::iter::PyDictIterator;
use std::io::Write;
use std::{fs, ptr::null};
use std::collections::HashMap;
use std::ptr;
use std::str::FromStr;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

enum TokenType {
    Null,
    List,
    Dict,
    String,
    Number,
    Bool,
    CustomType,
    BuiltinType,
}

const QUOTE_CHARS: [u8; 2] = [
    0x22, // "
    0x27, // '
    ];

const WHITESPACE_CHARS: [u8; 4] = [
    0x20, // ' '
    0x0A, // '\n'
    0x09, // '\t'
    0x0D, // '\r'
    ];

const AFTER_NULL_CHARS: [u8; 3] = [
    0x7D, // }
    0x5D, // ]
    0x2C, // ,
    ];


struct JsonWrapper {
    content: Vec<u8>,
    index: usize,
}

trait JsonWrapperTrait {
    fn slice(&self) -> &[u8];
    fn current(&self) -> Option<u8>;
}

impl JsonWrapperTrait for JsonWrapper {

    fn slice(&self) -> &[u8] {
        return &self.content[self.index..];
    }

    fn current(&self) -> Option<u8> {
        return Some(self.content[self.index]);
    }
}

impl Iterator for JsonWrapper {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.content[self.index];
        self.index += 1;
        return Some(c);
    }
}

fn skip_whitespace(json_wrapper: &mut JsonWrapper) {
    let mut index = json_wrapper.index;
    for c in json_wrapper.slice() {
        if WHITESPACE_CHARS.contains(&c) {
            index += 1;
            continue;
        }
        break;
    }

    if index > json_wrapper.index {
        println!("Shifting index from {} to {} at char \"{}\"({})", json_wrapper.index, index, json_wrapper.current().unwrap() as char, json_wrapper.current().unwrap());
        json_wrapper.index = index;
    }
}

fn find_key(json_wrapper: &mut JsonWrapper) -> String {

    let mut quote_char: u8 = 0x00;
    let mut index: usize = json_wrapper.index;
    for c in json_wrapper.slice() { 
        index += 1;
        if QUOTE_CHARS.contains(&c) {
            quote_char = *c;
            break;
        }
    }
    json_wrapper.index = index;


    let mut key:Vec<u8>  = Vec::new();
    let mut escaped: bool = false;

    for c in json_wrapper.slice() {
        index += 1;
        // If already escaped we just push the char and reset the flag
        if escaped {
            key.push(*c);
            escaped = false;
            continue;
        }
        // If we find the escape char we set the flag and continue
        if *c == 0x005C {
            key.push(*c);
            escaped = true;
            continue;
        }
        // If we find the quote char we reached the end of the key
        if *c == quote_char {
            break;
        }
        key.push(*c);
        continue;
    }
    json_wrapper.index = index;

    let key_string = String::from_utf8(key).unwrap();

    // Check that next non-whitespace character is a colon
    skip_whitespace(json_wrapper);
    let colon = json_wrapper.next().unwrap();
    if !(colon == 0x3A) {
        panic!("Unexpected  character \"{}\"({}) at index {}", colon as char, colon, json_wrapper.index )
    }

    

    return key_string;

}

fn handle_dict(json_wrapper: &mut JsonWrapper) {
    println!("Found a dict");
    skip_whitespace(json_wrapper);
    let key = find_key(json_wrapper);
    println!("Found key: {:?}", key);
    skip_whitespace(json_wrapper);

    let max: u16 = 10;
    let mut loop_index: u16 = 0;
    
    loop { 
        loop_index += 1;
        skip_whitespace(json_wrapper);
        let x = json_wrapper.current().unwrap();

        
        if x == 0x7D {
            println!("Found dict end at index {}", json_wrapper.index);
            break;
        }
        
        if max <= loop_index {
            println!("Reached max loops: {} at index {} with char \"{}\"({})", max, json_wrapper.index, x as char, x);
            break;
        }

        if x == 0x2C {
            json_wrapper.next();
            println!("Found a comma at index {}", json_wrapper.index);
            skip_whitespace(json_wrapper);
            let key = find_key(json_wrapper);
            println!("Found key: {:?}", key);
        }
        
        skip_whitespace(json_wrapper);

        handle_any(json_wrapper);
    }
}

fn handle_list(json_wrapper: &mut JsonWrapper) {
    println!("Found a list");
    skip_whitespace(json_wrapper);

    let max: u16 = 10;
    let mut index: u16 = 0;

    loop { 
        index += 1;
        let x = json_wrapper.current().unwrap();

        
        if x == 0x5D {
            println!("Found list end at index {}",json_wrapper.index);
            json_wrapper.next();
            break;
        }
        
        if max <= index {
            println!("Reached max loops: {} at index {} with char \"{}\"({})", max, json_wrapper.index, x as char, x);
            break;
        }

        skip_whitespace(json_wrapper);
        if x == 0x2C {
            json_wrapper.next();
            println!("Found a comma at index {}", json_wrapper.index);
        }
        
        skip_whitespace(json_wrapper);

        handle_any(json_wrapper);
    }
}


fn handle_dict_or_list(json_wrapper: &mut JsonWrapper) {
    match json_wrapper.current().unwrap() {
        0x7B => {
            handle_dict(json_wrapper)
        },
        0x5B => {
            json_wrapper.next();
            handle_list(json_wrapper)
        },
        _ => {
            panic!("Expected a dict or list but instead found \"{}\"({}) at index {}", json_wrapper.current().unwrap() as char, json_wrapper.current().unwrap(), json_wrapper.index);
        }
    }
}

fn handle_string(json_wrapper: &mut JsonWrapper, quote_char: u8) -> String {
    println!("Found a string");
    let mut escaped: bool = false;
    let mut value: Vec<u8> = Vec::new();
    let mut index: usize = json_wrapper.index;
    for c in json_wrapper.slice() {
        index += 1;
        if !escaped && *c == quote_char {
            println!("Found string end at index {}", index);
            break;
        }
        if !escaped {
            value.push(*c);
        }
        if *c == 0x5C {
            escaped = true;
        }
        continue;
    }

    json_wrapper.index = index;
    let str_value = String::from_utf8(value).unwrap();
    println!("Extracted string: \"{}\"", str_value);
    return str_value
}

fn handle_number(json_wrapper: &mut JsonWrapper) -> f32 {
    println!("Found a number");
    let mut value: Vec<u8> = Vec::new();
    let mut index: usize = json_wrapper.index;

    let max = index + 32;
 
    for c in json_wrapper.slice() {
        if WHITESPACE_CHARS.contains(&*c) {
            break;
        }
        if *c == 0x2C {
            break;
        }
        value.push(*c);
        index += 1;
    }

    json_wrapper.index = index;

    let str_value = String::from_utf8(value).unwrap();
    let num_value = f32::from_str(&str_value).unwrap();

    println!("Extracted float: \"{}\"", num_value);
    return num_value;

}

fn handle_custom_type(json_wrapper: &mut JsonWrapper) -> (String, String) {
    println!("Found a custom type");

    let mut type_id: Vec<u8> = Vec::new();
    let mut quote_char: u8 = 0x00;

    for c in json_wrapper.slice() {
        if WHITESPACE_CHARS.contains(&c) {
            panic!("Expected a custom type but instead found whitespace {} at index {}", c, json_wrapper.index);
        }
        if QUOTE_CHARS.contains(&c) {
            quote_char = *c;
            break;
        }
        type_id.push(*c);
    }

    json_wrapper.index += type_id.len();

    let type_id_str = String::from_utf8(type_id).unwrap();

    json_wrapper.index += 1;
    let value = handle_string(json_wrapper, quote_char);
    
    println!("Found custom type type \"{}\" with value \"{}\" ", type_id_str, value);
    return (value, type_id_str);
}

fn handle_null(json_wrapper: &mut JsonWrapper) {
    println!("Suspecting a null");
    //let mut index: usize = json_wrapper.index;
    //let mut type_id: Vec<u8> = Vec::new();

    for c in b"null" {
        if !(json_wrapper.current().unwrap() == *c) {
            panic!("Expected a null but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
        }
        json_wrapper.next();
    }


    skip_whitespace(json_wrapper);
    if !(AFTER_NULL_CHARS.contains(&(json_wrapper.current().unwrap()))) {
        panic!("Expected a null but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
    }

    println!("Found a null");
    
}

fn handle_bool(json_wrapper: &mut JsonWrapper, _true: bool) {
    if _true {
        println!("Suspecting true (bool)");
        for c in b"true" {
            if !(json_wrapper.current().unwrap() == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }
    else {
        println!("Suspecting false (bool)");
        for c in b"false" {
            if !(json_wrapper.current().unwrap() == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }   
}

fn handle_any(json_wrapper: &mut JsonWrapper) {
    let c = json_wrapper.current().unwrap();
    println!("Found something starting with {}({}) at index {}", c, c as char, json_wrapper.index);
    match c {
        // " | '
        0x0022 | 0x0027  => {
            json_wrapper.next(); // Skipping the quote char
            handle_string(json_wrapper, c);
        },
        // Numbers
        0x2E | 0x0030..=0x0039 => {
            handle_number(json_wrapper);
        },
        // Null
        0x6e => {
            handle_null(json_wrapper);
        },
        // Bool: false
        0x66 => {
            handle_bool(json_wrapper, false);
        }
        // Bool: true
        0x74 => {
            handle_bool(json_wrapper, true);
        }
        // Custom
        0x40 => {
            handle_custom_type(json_wrapper);
        },
        _ => {
            handle_dict_or_list(json_wrapper)
        }
    }

}



#[pyclass]
struct ValueEnvelope{
    #[pyo3(get, set)]
    value: String,
    #[pyo3(get, set)]
    class: String
}




/// Formats the sum of two numbers as string.
#[pyfunction]
fn load_file(file_path: String) -> PyResult<HashMap<String, ValueEnvelope>> {

    let contents = fs::read(file_path)
    .expect("Should have been able to read the file");

    let mut json_wrapper = JsonWrapper {
        content: contents,
        index: 0,
    };


    let dict: HashMap<String, ValueEnvelope> = HashMap::new();




    // Skip forward the first non-whitespace character
    skip_whitespace(&mut json_wrapper);
    handle_dict_or_list(&mut json_wrapper);
    


    Ok(dict)
}


#[pyfunction]
fn loads(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn dump(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn dumps(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn magicjson_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}
