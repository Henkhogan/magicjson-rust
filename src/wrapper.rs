use crate::constants::{COLON_CHAR, ESCAPE_CHAR, QUOTE_CHARS, WHITESPACE_CHARS, LOOP_MAX_ITERATIONS};

use std::{io::BufReader, fs::File};

use log::{debug, trace};

use crate::JsonKey;

pub struct JsonBytesWrapper {
    pub bufreader: std::io::Bytes<BufReader<File>>,
    pub index: usize,
    pub current: u8,
    pub end_reached: bool,
}

pub trait JsonWrapperTrait {
    fn skip_whitespace(&mut self);
    fn skip_colon(&mut self);
    fn find_key(&mut self) -> String;
}

impl JsonWrapperTrait for JsonBytesWrapper{

    fn skip_whitespace(&mut self) {

        let start_index = self.index;
        trace!("Entering skip_whitespace at index {} with char: {}", start_index, self.current as char);

        let mut _lix = 0;
        loop {
            _lix += 1;
            if _lix >= LOOP_MAX_ITERATIONS {
                panic!("Reached max iterations while skipping whitespace");
            }
            if self.end_reached {
                break;
            }
            if !WHITESPACE_CHARS.contains(&self.current) {
                break;
            }
            self.next();
        }   
  
        if self.index > start_index {
            trace!("(Skip Whitespace) Shifted index from {} to {} at char \"{}\"({})", start_index, self.index, self.current as char, self.current);
        }
    }

    fn skip_colon(&mut self) {
        if self.current != COLON_CHAR {
            panic!("Expected a colon but instead found \"{}\"({}) at index {}", self.current as char, self.current, self.index);
        }
        trace!("Found a colon at index {}", self.index);
        self.next();
        trace!("(Skip colon) Shifted index to {}", self.index);
    }

    fn find_key(&mut self) -> JsonKey {

        trace!("Searching key starting at index {}", self.index);

        let quote_char: u8;

        let mut c: u8 = self.current;

        let mut _lix: u16 = 0;
        loop {
            trace!("Checking char: {} at index {}", c as char, self.index);
            _lix += 1;
            if _lix >= LOOP_MAX_ITERATIONS {
                panic!("Reached max iterations while searching for quote char");
            }
            if QUOTE_CHARS.contains(&c) {
                quote_char = c.clone();
                trace!("Detected quote char: {} at index {}", c as char, self.index);
                break;
            }
            c = self.next().unwrap();

        }
 
        let mut key:JsonKey = JsonKey::new();

        _lix = 0;
        loop {
            _lix += 1;
            if _lix >= LOOP_MAX_ITERATIONS {
                panic!("Reached max iterations while searching for quote char");
            }
            c = self.next().unwrap();

            match c {
                // If we find the escape char we read the next 2 bytes and shift the index by 1
                ESCAPE_CHAR => {
                    trace!("Found escape char at index {}", self.index);
                    key.push(c as char);
                    key.push(self.next().unwrap() as char);
                    self.next();
                    continue;
                },
                // If we find the quote char we reached the end of the key
                c if c == quote_char => {
                    trace!("Found end of key {} at index {}", c as char, self.index);
                    self.next();
                    break;
                },
                _ => {
                    key.push(c as char);
                    continue;
                }  
            }
        }
        return key
    }
}


impl Iterator for JsonBytesWrapper {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.bufreader.next() {
            Some(Ok(c)) => {
                self.current = c;
                return Some(c);
            },
            Some(Err(e)) => {
                panic!("Error while reading file: {}", e);
            },
            None => {
                debug!("(Next) Reached end of file at index {}", self.index);
                self.end_reached = true;
                return None;
            }
        }
    }
}