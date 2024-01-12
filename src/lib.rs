use pyo3::{prelude::*, ffi::PyObject};
//use pyo3::AsPyPointer;
use std::fmt;
//use pyo3::types::PyDict;
//use pyo3::types::iter::PyDictIterator;
use std::io::Write;
use std::fs;
//use std::str::FromStr;
use log::{debug, info};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

mod wrapper;
use wrapper::{JsonBytesWrapper, JsonWrapperTrait};


mod constants;
use constants::{DIGIT_CHARS, WHITESPACE_CHARS, QUOTE_CHARS, DICT_START_CHAR, DICT_END_CHAR, LIST_START_CHAR, LIST_END_CHAR, COMMA_CHAR, DOT_CHAR, MINUS_CHAR, PLUS_CHAR, ESCAPE_CHAR, AFTER_NULL_CHARS, LOOP_MAX_ITERATIONS, MAX_ITEMS, NUMERIC_CHARS};

#[pyclass(module="magicjson")]
#[derive(Clone, Copy, Debug)]
enum JsonType {
    Null,
    List,
    Dict,
    String,
    Int,
    Float,
    Bool,
    CustomType,
}

impl fmt::Display for JsonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {            
            JsonType::Null => write!(f, "Null"),
            JsonType::List => write!(f, "List"),
            JsonType::Dict => write!(f, "Dict"),
            JsonType::String => write!(f, "String"),
            JsonType::Int => write!(f, "Int"),
            JsonType::Float => write!(f, "Float"),
            JsonType::Bool => write!(f, "Bool"),
            JsonType::CustomType => write!(f, "CustomType"),
        }
    }
}



#[pyclass(module="magicjson", get_all)]
#[derive(Clone, Debug)]
struct JsonItem {
    key: Option<String>,
    value: Option<Vec<u8>>,
    items: Option<Vec<JsonItem>>,
    value_type: JsonType,
    value_custom_type: Option<String>,
}


//impl IntoPy<PyObject> for JsonWrapper {
//    fn into_py(self, py: Python<'_>) -> PyObject {
//        pyo3::IntoPy::into_py(pyo3::Py::new(py, self).unwrap(), py)
//
//        //self.top_level_type.into_py(py),
//        //self.children.unwrap().into_py(py)
//    }
//}


fn handle_dict(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    debug!("Found a dict");
    json_wrapper.skip_whitespace();
    let mut ikey = json_wrapper.find_key();
    info!("Found key: {:?}", key);
    json_wrapper.skip_whitespace();
    json_wrapper.skip_colon();
    json_wrapper.skip_whitespace();

    let mut values: Vec<JsonItem> = Vec::new();

    let mut loop_index: u16 = 0;
    
    loop { 
        loop_index += 1;
        json_wrapper.skip_whitespace();
        let x = json_wrapper.current().unwrap();
        
        if x == DICT_END_CHAR {
            log::info!("Found dict end at index {}", json_wrapper.index);
            break;
        }
        
        if MAX_ITEMS <= loop_index {
            log::warn!("Reached max loops: {} at index {} with char \"{}\"({})", MAX_ITEMS, json_wrapper.index, x as char, x);
            break;
        }

        if x == COMMA_CHAR {
            json_wrapper.next();
            log::debug!("(Dict) Found a comma at index {}", json_wrapper.index);
            json_wrapper.skip_whitespace();
            if json_wrapper.current().unwrap() == DICT_END_CHAR {
                log::warn!("Found a comma followed by a dict end at index {}", json_wrapper.index);
                json_wrapper.skip_whitespace();
                if json_wrapper.end_reached() {
                    log::warn!("Reached end of file");
                    break;
                }
            }
            ikey = json_wrapper.find_key();
            log::info!("Found key: {:?}", ikey);
            json_wrapper.skip_whitespace();
            json_wrapper.skip_colon();
            continue;
        }
        
        json_wrapper.skip_whitespace();
        values.push(handle_any(json_wrapper, Some(ikey.clone())));  
    }
    
    return JsonItem {
        key: key,
        value: None,
        items: Some(values),
        value_type: JsonType::Dict,
        value_custom_type: None,
    }
}

fn handle_list(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    log::info!("Processing a list");
    json_wrapper.skip_whitespace();

    let mut values: Vec<JsonItem> = Vec::new();

    let max: u16 = 10;
    let mut index: u16 = 0;

    loop { 
        index += 1;
        let x = json_wrapper.current().unwrap();

        
        if x == LIST_END_CHAR {
            println!("Found list end at index {}",json_wrapper.index);
            json_wrapper.next();
            break;
        }
        
        if max <= index {
            println!("Reached max loops: {} at index {} with char \"{}\"({})", max, json_wrapper.index, x as char, x);
            break;
        }

        json_wrapper.skip_whitespace();
        if x == 0x2C {
            json_wrapper.next();
            println!("(List) Found a comma at index {}", json_wrapper.index);
        }
        
        json_wrapper.skip_whitespace();

        values.push(handle_any(json_wrapper, None));
    }

    return JsonItem {
        key: key,
        value: None,
        items: Some(values),
        value_type: JsonType::List,
        value_custom_type: None,
    }
}


fn handle_dict_or_list(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    match json_wrapper.current().unwrap() {
        DICT_START_CHAR => {
            return handle_dict(json_wrapper, key);
        },
        LIST_START_CHAR => {
            json_wrapper.next();
            return handle_list(json_wrapper, key);
        },
        _ => {
            panic!("Expected a dict or list but instead found \"{}\"({}) at index {}", json_wrapper.current().unwrap() as char, json_wrapper.current().unwrap(), json_wrapper.index);
        }
    }
}

fn handle_string(json_wrapper: &mut JsonBytesWrapper, quote_char: u8) -> Vec<u8> {

    log::debug!("Processing a string");
    let mut value: Vec<u8> = Vec::new();
    let mut c: u8;

    loop {
        c = json_wrapper.current().unwrap();

        

        if c == ESCAPE_CHAR {
            debug!("Found escape char at index {}", json_wrapper.index);
            value.push(c);
            value.push(json_wrapper.next().unwrap());
            json_wrapper.next();
            continue;
        }

        if c == quote_char {
            log::debug!("Found string end at index {}", json_wrapper.index);
            json_wrapper.next();
            break;
        }

        value.push(c);
        json_wrapper.next();

    }

    // for c in json_wrapper.slice() {
    //     index += 1;
    //     if !escaped && *c == quote_char {
    //         log::debug!("Found string end at index {}", index);
    //         break;
    //     }
    //     if !escaped {
    //         value.push(*c);
    //     }
    //     if *c == ESCAPE_CHAR {
    //         escaped = true;
    //         debug!("Found escape char at index {}", index);
    //         value.push(*c);
    //     }
    //     continue;
    // }

    // json_wrapper.index = index;
    //let str_value = String::from_utf8(value).unwrap();
    log::info!("Extracted string: \"{}\"", std::str::from_utf8(&value).unwrap());
    return value;
}

fn handle_number(json_wrapper: &mut JsonBytesWrapper) -> (JsonType, Vec<u8>) {
    log::debug!("Handling a number at index {}", json_wrapper.index);
    let mut value: Vec<u8> = Vec::new();
    let mut is_float: bool = false;
    let mut is_signed: bool = false;
    
    //let mut c = json_wrapper.current().unwrap();
    let mut c: u8 = json_wrapper.current().unwrap();

    if !NUMERIC_CHARS.contains(&c) || c == MINUS_CHAR | PLUS_CHAR {
        panic!("Expected a number but instead found \"{}\"({}) at index {}", c as char, c, json_wrapper.index);
    }

    let mut _lix: u16 = 0;
    loop {
        //c = json_wrapper.current().unwrap();

        _lix += 1;
        if _lix >= LOOP_MAX_ITERATIONS {
            panic!("Reached max iterations {} at index {}", LOOP_MAX_ITERATIONS, json_wrapper.index);
        }

        c = json_wrapper.current().unwrap();

        if c == DOT_CHAR {
            if is_float {
                panic!("(Number) Found repeated dot at index {}", json_wrapper.index);
            }
            is_float = true;
        }

        if c == MINUS_CHAR | PLUS_CHAR {
            if is_signed {
                panic!("(Number) Found repeated sign \"{}\" at index {}", c as char, json_wrapper.index);
            }
            is_signed = true;
        }

        if !NUMERIC_CHARS.contains(&c) {
            break;
        }
        
        value.push(c);
        json_wrapper.next();
    }


    log::info!("Extracted number: \"{}\". Float: {}. Current index: {}", std::str::from_utf8(&value).unwrap(), is_float, json_wrapper.index);
    if is_float {
        return (JsonType::Float, value);
    }
    return (JsonType::Int, value);

}

fn handle_custom_type(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    log::debug!("Processing a custom type");

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
    
    log::info!("Found custom type type \"{}\" with value \"{}\" ", type_id_str, std::str::from_utf8(&value).unwrap());
    return JsonItem {
        key: key,
        value: Some(value),
        items: None,
        value_type: JsonType::CustomType,
        value_custom_type: Some(type_id_str),
    };
}

fn handle_null(json_wrapper: &mut JsonBytesWrapper) {
    log::debug!("Suspecting a null");
    //let mut index: usize = json_wrapper.index;
    //let mut type_id: Vec<u8> = Vec::new();

    for c in b"null" {
        if !(json_wrapper.current().unwrap() == *c) {
            panic!("Expected a null but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
        }
        json_wrapper.next();
    }


    json_wrapper.skip_whitespace();
    if !(AFTER_NULL_CHARS.contains(&(json_wrapper.current().unwrap()))) {
        panic!("Expected a null but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
    }

    log::info!("Found a null");
    
}

fn handle_bool(json_wrapper: &mut JsonBytesWrapper, _true: bool) {
    if _true {
        log::debug!("Suspecting true (bool)");
        for c in b"true" {
            if !(json_wrapper.current().unwrap() == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }
    else {
        log::debug!("Suspecting false (bool)");
        for c in b"false" {
            if !(json_wrapper.current().unwrap() == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }   
}

fn handle_any(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    let c = json_wrapper.current().unwrap();
    log::debug!("Found something starting with {}({}) at index {}", c, c as char, json_wrapper.index);
    match c {
        // " | '
        0x0022 | 0x0027  => {
            json_wrapper.next(); // Skipping the quote char
            return JsonItem {
                key: key,
                value: Some(handle_string(json_wrapper, c)),
                items: None,
                value_type: JsonType::String,
                value_custom_type: None,
            }
        },
        // Numbers - ToDo: Use arrays
        _ if DIGIT_CHARS.contains(&c) || [DOT_CHAR | MINUS_CHAR | PLUS_CHAR].contains(&c) => {
            let x = handle_number(json_wrapper);
            return JsonItem {
                key: key,
                value: Some(x.1),
                items: None,
                value_type: x.0,
                value_custom_type: None,
            };
        },
        // Null
        0x6e => {
            handle_null(json_wrapper);
            return JsonItem {
                key: key,
                value: None,
                items: None,
                value_type: JsonType::Null,
                value_custom_type: None,
            };
        },
        // Bool: false
        0x66 => {
            handle_bool(json_wrapper, false);
            return JsonItem {
                key: key,
                value: Some(b"0".to_vec()),
                items: None,
                value_type: JsonType::Bool,
                value_custom_type: None,
            };
            
        }
        // Bool: true
        0x74 => {
            handle_bool(json_wrapper, true);
            return JsonItem {
                key: key,
                value: Some(b"1".to_vec()),
                items: None,
                value_type: JsonType::Bool,
                value_custom_type: None,
            };
        }
        // Custom
        0x40 => {
            return handle_custom_type(json_wrapper, key);            
        },
        _ => {
            handle_dict_or_list(json_wrapper, key)
        }
    }

}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn load_file(file_path: String) -> JsonItem {

    //let contents = fs::read(file_path)
    //.expect("Should have been able to read the file");

    let mut json_wrapper = JsonBytesWrapper {
        content: fs::read(file_path).unwrap(),
        index: 0,
    };



    // Skip forward the first non-whitespace character
    json_wrapper.skip_whitespace();
    
    let top_level_item = handle_dict_or_list(&mut json_wrapper, None);
    
    match top_level_item.value_type {
        JsonType::Dict | JsonType::List => {
            info!("Returning {} with {} items", top_level_item.value_type, top_level_item.items.as_ref().unwrap().len());
            return top_level_item
        },
        __cause__ => {
            panic!("Expected a dict or list but instead found \"{}\" at index {}", json_wrapper.current().unwrap() as char, json_wrapper.index);
        }
    }
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
fn magicjson(_py: Python, m: &PyModule) -> PyResult<()> {
    
    Builder::new()
    .format(|buf, record| {
        writeln!(buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter(None, LevelFilter::Debug)
    .init();

    //pyo3_log::init();
    m.add_class::<JsonItem>()?;
    m.add_class::<JsonType>()?;

    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}
