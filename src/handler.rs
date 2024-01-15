use std::collections::HashMap;

use log::{debug, info};



use crate::constants::{DIGIT_CHARS, WHITESPACE_CHARS, QUOTE_CHARS, DICT_START_CHAR, DICT_END_CHAR, LIST_START_CHAR, LIST_END_CHAR, COMMA_CHAR, DOT_CHAR, MINUS_CHAR, PLUS_CHAR, ESCAPE_CHAR, AFTER_NULL_CHARS, LOOP_MAX_ITERATIONS, MAX_ITEMS, NUMERIC_CHARS};

use crate::objects::{JsonType, JsonItem, JsonKey, JsonCustomType};

use crate::wrapper::{JsonBytesWrapper, JsonWrapperTrait};



pub fn handle_dict(json_wrapper: &mut JsonBytesWrapper) -> JsonItem {

    debug!("Found a dict");
    json_wrapper.skip_whitespace();
    let mut ikey = json_wrapper.find_key();
    info!("Found key: {:?}", ikey);
    json_wrapper.skip_whitespace();
    json_wrapper.skip_colon();
    json_wrapper.skip_whitespace();

    let mut values: HashMap<JsonKey, JsonItem> = HashMap::new();

    let mut c: u8;
    let mut _lix: u16 = 0;


    
    loop { 
        json_wrapper.skip_whitespace();
        c = json_wrapper.current;
        
        _lix += 1;
        if MAX_ITEMS <= _lix {
            log::warn!("Reached max loops: {} at index {} with char \"{}\"({})", MAX_ITEMS, json_wrapper.index, c as char, c);
            break;
        }
        
        match c {
            DICT_END_CHAR => {
                log::info!("Found dict end at index {}", json_wrapper.index);
                break;
            },
            COMMA_CHAR => {
                json_wrapper.next();
                log::debug!("(Dict) Found a comma at index {}", json_wrapper.index);
                json_wrapper.skip_whitespace();
                if json_wrapper.current == DICT_END_CHAR {
                    log::warn!("Found a comma followed by a dict end at index {}", json_wrapper.index);
                    json_wrapper.next();
                    json_wrapper.skip_whitespace();
                }
                if json_wrapper.end_reached {
                    log::warn!("Reached end of file with current key \"{}\" at index {}", ikey, json_wrapper.index);
                    break;
                }
                ikey = json_wrapper.find_key();
                log::info!("Found key: {:?}", ikey);
                json_wrapper.skip_whitespace();
                json_wrapper.skip_colon();
                continue;
            },
            _ => {}
            
        }
      
        json_wrapper.skip_whitespace();
        values.insert(ikey.clone(), handle_any(json_wrapper, Some(ikey.clone()))); 
        debug!("Inserted key \"{}\" at index {}", ikey, json_wrapper.index);
        //values.push(handle_any(json_wrapper, Some(ikey.clone())));  
    }
    
    return JsonItem::Dict(values);  
}

pub fn handle_list(json_wrapper: &mut JsonBytesWrapper) -> JsonItem {
    log::info!("Processing a list");
    json_wrapper.skip_whitespace();

    let mut values: Vec<JsonItem> = Vec::new();

    let max: u16 = 10;
    let mut _lix: u16 = 0;

    loop { 
        _lix += 1;
        let x = json_wrapper.current;

        
        if x == LIST_END_CHAR {
            info!("Found list end at index {}",json_wrapper.index);
            json_wrapper.next();
            break;
        }
        
        if max <= _lix {
            panic!("Reached max loops: {} at index {} with char \"{}\"({})", max, json_wrapper.index, x as char, x);
            
        }

        json_wrapper.skip_whitespace();
        if x == 0x2C {
            json_wrapper.next();
            info!("(List) Found a comma at index {}", json_wrapper.index);
        }
        
        json_wrapper.skip_whitespace();

        values.push(handle_any(json_wrapper, None));
    }

    return JsonItem::List(values);
}


pub fn handle_dict_or_list(json_wrapper: &mut JsonBytesWrapper) -> JsonItem {
    match json_wrapper.current {
        DICT_START_CHAR => {
            return handle_dict(json_wrapper);
        },
        LIST_START_CHAR => {
            json_wrapper.next();
            return handle_list(json_wrapper);
        },
        _ => {
            panic!("Expected a dict or list but instead found \"{}\"({}) at index {}", json_wrapper.current as char, json_wrapper.current, json_wrapper.index);
        }
    }
}

fn handle_string(json_wrapper: &mut JsonBytesWrapper, quote_char: u8) -> String {

    log::debug!("Processing a string");
    //let mut value: Vec<u8> = Vec::new();
    let mut value_str: String = String::new();
    let mut c: u8;

    loop {
        c = json_wrapper.current;

        

        if c == ESCAPE_CHAR {
            debug!("Found escape char at index {}", json_wrapper.index);

            value_str.push(c as char);
            value_str.push(json_wrapper.next().unwrap() as char);
            json_wrapper.next();
            continue;
        }

        if c == quote_char {
            log::debug!("Found string end at index {}", json_wrapper.index);
            json_wrapper.next();
            break;
        }

        value_str.push(c as char);
        json_wrapper.next();

    }

    log::info!("Extracted string: \"{}\"", value_str);
    return value_str;
}

fn handle_number(json_wrapper: &mut JsonBytesWrapper) -> JsonItem {
    log::debug!("Handling a number at index {}", json_wrapper.index);
    let mut value: Vec<u8> = Vec::new();
    let mut is_float: bool = false;
    let mut is_signed: bool = false;
    
    //let mut c = json_wrapper.current;
    let mut c: u8 = json_wrapper.current;

    if !NUMERIC_CHARS.contains(&c) || c == MINUS_CHAR | PLUS_CHAR {
        panic!("Expected a number but instead found \"{}\"({}) at index {}", c as char, c, json_wrapper.index);
    }

    let mut _lix: u16 = 0;
    loop {
        //c = json_wrapper.current;

        _lix += 1;
        if _lix >= LOOP_MAX_ITERATIONS {
            panic!("Reached max iterations {} at index {}", LOOP_MAX_ITERATIONS, json_wrapper.index);
        }

        c = json_wrapper.current;

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

    let value_str = std::str::from_utf8(&value).unwrap();
    log::info!("Extracted number: \"{}\". Float: {}. Current index: {}", value_str, is_float, json_wrapper.index);
    if is_float {
        return JsonItem::Float(value_str.parse().unwrap());    
    }
    return JsonItem::Int(value_str.parse().unwrap());

}

fn handle_custom_type(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    log::debug!("Processing a custom type");

    let mut type_id: Vec<u8> = Vec::new();
    let quote_char: u8;

    let mut c = json_wrapper.current;
    let mut _lix: u16 = 0;
    loop {
        _lix += 1;
        if _lix >= LOOP_MAX_ITERATIONS {
            panic!("Reached max iterations {} at index {}", LOOP_MAX_ITERATIONS, json_wrapper.index);
        }
        if WHITESPACE_CHARS.contains(&c) {
            panic!("Expected a custom type but instead found whitespace {} at index {}", c, json_wrapper.index);
        }
        if QUOTE_CHARS.contains(&c) {
            quote_char = c;
            break;
        }
        type_id.push(c);
        c = json_wrapper.next().unwrap();
    }


    let type_id_str = String::from_utf8(type_id).unwrap();

    json_wrapper.next();
    let value = handle_string(json_wrapper, quote_char);
    
    log::info!("Found custom type type \"{}\" with value \"{}\" ", type_id_str, value);

    return JsonItem::Custom(JsonCustomType{name: type_id_str, value: value});
}

fn handle_null(json_wrapper: &mut JsonBytesWrapper) {
    log::debug!("Suspecting a null");
    //let mut index: usize = json_wrapper.index;
    //let mut type_id: Vec<u8> = Vec::new();

    for c in b"null" {
        if !(json_wrapper.current == *c) {
            panic!("Expected a null but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
        }
        json_wrapper.next();
    }


    json_wrapper.skip_whitespace();
    if !(AFTER_NULL_CHARS.contains(&(json_wrapper.current))) {
        panic!("Expected a null but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
    }

    log::info!("Found a null");
}

fn handle_bool(json_wrapper: &mut JsonBytesWrapper, _true: bool) {
    if _true {
        log::debug!("Suspecting true (bool)");
        for c in b"true" {
            if !(json_wrapper.current == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }
    else {
        log::debug!("Suspecting false (bool)");
        for c in b"false" {
            if !(json_wrapper.current == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }   
}

fn handle_any(json_wrapper: &mut JsonBytesWrapper, key: Option<String>) -> JsonItem {
    let c = json_wrapper.current;
    log::debug!("Found something starting with {}({}) at index {}", c, c as char, json_wrapper.index);
    match c {
        // " | '
        0x0022 | 0x0027  => {
            json_wrapper.next(); // Skipping the quote char
            return JsonItem::Str(handle_string(json_wrapper, c));
        },
        // Numbers - ToDo: Use arrays
        _ if DIGIT_CHARS.contains(&c) || [DOT_CHAR | MINUS_CHAR | PLUS_CHAR].contains(&c) => {
            return handle_number(json_wrapper);
        },
        // Null
        0x6e => {
            handle_null(json_wrapper);
            return JsonItem::Null();
        },
        // Bool: false
        0x66 => {
            handle_bool(json_wrapper, false);
            return JsonItem::Bool(false);
        }
        // Bool: true
        0x74 => {
            handle_bool(json_wrapper, true);
            return JsonItem::Bool(true);
        }
        // Custom
        0x40 => {
            return handle_custom_type(json_wrapper, key);            
        },
        _ => {
            return handle_dict_or_list(json_wrapper)
        }
    }

}