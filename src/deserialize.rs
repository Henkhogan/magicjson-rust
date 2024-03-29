use std::collections::HashMap;

use log::{debug, trace};



use crate::constants::{DIGIT_CHARS, WHITESPACE_CHARS, QUOTE_CHARS, DICT_START_CHAR, DICT_END_CHAR, LIST_START_CHAR, LIST_END_CHAR, COMMA_CHAR, DOT_CHAR, MINUS_CHAR, PLUS_CHAR, ESCAPE_CHAR, AFTER_NULL_CHARS, LOOP_MAX_ITERATIONS, MAX_ITEMS, NUMERIC_CHARS, DATETIME_ID, TIMESTAMP_ID};

use crate::objects::{JsonItem, JsonKey, JsonCustomType};

use crate::wrapper::{JsonBytesWrapper, JsonWrapperTrait};

//use iso8601::{DateTime as IsoDateTime};
use chrono::DateTime;

pub fn handle_dict<T>(json_wrapper: &mut JsonBytesWrapper) -> T where T: From<JsonItem>{

    trace!("Found a dict");
    json_wrapper.skip_whitespace();
    let mut ikey = json_wrapper.find_key();
    debug!("Found key: {:?}", ikey);
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
                log::debug!("Found dict end at index {}", json_wrapper.index);
                break;
            },
            COMMA_CHAR => {
                json_wrapper.next();
                log::trace!("(Dict) Found a comma at index {}", json_wrapper.index);
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
                log::debug!("Found key: {:?}", ikey);
                json_wrapper.skip_whitespace();
                json_wrapper.skip_colon();
                continue;
            },
            _ => {}
            
        }
      
        json_wrapper.skip_whitespace();
        values.insert(ikey.clone(), handle_any(json_wrapper)); 
        trace!("Inserted key \"{}\" at index {}", ikey, json_wrapper.index);
    }
    return T::from(JsonItem::Dict(values));
}

pub fn handle_list<T>(json_wrapper: &mut JsonBytesWrapper) -> T where T: From<JsonItem> {
    log::debug!("Processing a list");
    json_wrapper.skip_whitespace();

    let mut values: Vec<JsonItem> = Vec::new();

    let max: u16 = 10;
    let mut _lix: u16 = 0;

    loop { 
        _lix += 1;
        let x = json_wrapper.current;

        
        if x == LIST_END_CHAR {
            debug!("Found list end at index {}",json_wrapper.index);
            json_wrapper.next();
            break;
        }
        
        if max <= _lix {
            panic!("Reached max loops: {} at index {} with char \"{}\"({})", max, json_wrapper.index, x as char, x);
            
        }

        json_wrapper.skip_whitespace();
        if x == 0x2C {
            json_wrapper.next();
            debug!("(List) Found a comma at index {}", json_wrapper.index);
        }
        
        json_wrapper.skip_whitespace();

        values.push(handle_any(json_wrapper));
    }
    return T::from(JsonItem::List(values));
}


pub fn handle_dict_or_list<T>(json_wrapper: &mut JsonBytesWrapper) -> T where T: From<JsonItem> {
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

    log::trace!("Processing a string");
    //let mut value: Vec<u8> = Vec::new();
    let mut value_str: String = String::new();
    let mut c: u8;

    loop {
        c = json_wrapper.current;

        

        if c == ESCAPE_CHAR {
            trace!("Found escape char at index {}", json_wrapper.index);

            value_str.push(c as char);
            value_str.push(json_wrapper.next().unwrap() as char);
            json_wrapper.next();
            continue;
        }

        if c == quote_char {
            log::trace!("Found string end at index {}", json_wrapper.index);
            json_wrapper.next();
            break;
        }

        value_str.push(c as char);
        json_wrapper.next();

    }

    log::debug!("Extracted string: \"{}\"", value_str);
    return value_str;
}

fn handle_number(json_wrapper: &mut JsonBytesWrapper) -> JsonItem {
    log::trace!("Handling a number at index {}", json_wrapper.index);
    let mut value: Vec<u8> = Vec::new();
    let mut is_float: bool = false;
    let mut is_signed: bool = false;
    

    if !NUMERIC_CHARS.contains(&json_wrapper.current) || json_wrapper.current == MINUS_CHAR | PLUS_CHAR {
        panic!("Expected a number but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
    }

    let mut _lix: u16 = 0;
    loop {
        //c = json_wrapper.current;

        _lix += 1;
        if _lix >= LOOP_MAX_ITERATIONS {
            panic!("Reached max iterations {} at index {}", LOOP_MAX_ITERATIONS, json_wrapper.index);
        }

        match json_wrapper.current {
            DOT_CHAR  => {
                if is_float {
                    panic!("(Number) Found repeated dot at index {}", json_wrapper.index);
                }
                is_float = true;
            },
            MINUS_CHAR | PLUS_CHAR => {
                if is_signed {
                    panic!("(Number) Found repeated sign \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
                }
                is_signed = true;
            },
            _ if !NUMERIC_CHARS.contains(&json_wrapper.current) => {
                break;
            },
            _ => {}
        }
        
        value.push(json_wrapper.current);
        json_wrapper.next();
    }

    let value_str = std::str::from_utf8(&value).unwrap();
    log::debug!("Extracted number: \"{}\". Float: {}. Current index: {}", value_str, is_float, json_wrapper.index);
    if is_float {
        return JsonItem::Float(value_str.parse().unwrap());    
    }
    return JsonItem::Int(value_str.parse().unwrap());

}

fn handle_custom_type<T>(json_wrapper: &mut JsonBytesWrapper) -> T where T: From<JsonItem>, T: From<JsonItem> {
    log::trace!("Processing a custom type");

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
    
    log::debug!("Found custom type type \"{}\" with value \"{}\" ", type_id_str, value);

    match type_id_str.as_str() {
        DATETIME_ID => {
            return T::from(JsonItem::Datetime(DateTime::parse_from_rfc3339(&value).unwrap().naive_utc()));
        },
        TIMESTAMP_ID => {
            return T::from(JsonItem::Timestamp(value.parse().unwrap()));
        },
        _ => {
            return T::from(JsonItem::Custom(JsonCustomType{name: type_id_str, value: value}));
        }
        
    }

}

fn handle_null(json_wrapper: &mut JsonBytesWrapper) {
    log::trace!("Suspecting a null");
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

    log::debug!("Found a null");
}

fn handle_bool(json_wrapper: &mut JsonBytesWrapper, _true: bool) {
    if _true {
        log::trace!("Suspecting true (bool)");
        for c in b"true" {
            if !(json_wrapper.current == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }
    else {
        log::trace!("Suspecting false (bool)");
        for c in b"false" {
            if !(json_wrapper.current == *c) {
                panic!("Expected tree (bool) but instead found \"{}\" at index {}", json_wrapper.current as char, json_wrapper.index);
            }
            json_wrapper.next();
        }
    }   
}

fn handle_any<T>(json_wrapper: &mut JsonBytesWrapper) -> T where T: From<JsonItem> {
    let c = json_wrapper.current;
    log::trace!("Found something starting with {}({}) at index {}", c, c as char, json_wrapper.index);
    match c {
        // " | '
        0x0022 | 0x0027  => {
            json_wrapper.next(); // Skipping the quote char
            return T::from(JsonItem::Str(handle_string(json_wrapper, c)));
        },
        // Numbers - ToDo: Use arrays
        _ if DIGIT_CHARS.contains(&c) || [DOT_CHAR | MINUS_CHAR | PLUS_CHAR].contains(&c) => {
            return T::from(handle_number(json_wrapper));
        },
        // Null
        0x6e => {
            handle_null(json_wrapper);
            return T::from(JsonItem::Null());
        },
        // Bool: false
        0x66 => {
            handle_bool(json_wrapper, false);
            return T::from(JsonItem::Bool(false));
        }
        // Bool: true
        0x74 => {
            handle_bool(json_wrapper, true);
            return T::from(JsonItem::Bool(true));
        }
        // Custom
        0x40 => {
            return handle_custom_type(json_wrapper);            
        },
        _ => {
            return handle_dict_or_list(json_wrapper)
        }
    }

}