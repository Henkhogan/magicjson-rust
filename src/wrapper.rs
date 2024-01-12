#[path = "./constants.rs"] mod constants;
use constants::*;
use log::debug;

pub struct JsonBytesWrapper {
    pub content: Vec<u8>,
    pub index: usize,
    //items: Vec<_JsonItem>,
}

pub trait JsonWrapperTrait {
    fn end_reached(&self) -> bool;
    fn slice(&self) -> &[u8];
    fn current(&self) -> Option<u8>;
    fn skip_whitespace(&mut self);
    fn skip_colon(&mut self);
    fn find_key(&mut self) -> String;
}

impl JsonWrapperTrait for JsonBytesWrapper{


    fn end_reached(&self) -> bool {
        return self.index+1 >= self.content.len();
    }
    fn slice(&self) -> &[u8] {
        return &self.content[self.index..];
    }

    fn current(&self) -> Option<u8> {
        return Some(self.content[self.index]);
    }

    fn skip_whitespace(&mut self) {

        let start_index = self.index;
        debug!("Entering skip_whitespace at index {} with char: {}", start_index, self.current().unwrap() as char);

        while WHITESPACE_CHARS.contains(&self.current().unwrap()) {
            self.next().unwrap();
        }
        
        if self.index > start_index {
            debug!("(Skip Whitespace) Shifted index from {} to {} at char \"{}\"({})", start_index, self.index, self.current().unwrap() as char, self.current().unwrap());
        }
    }

    fn skip_colon(&mut self) {
        if self.current().unwrap() != COLON_CHAR {
            panic!("Expected a colon but instead found \"{}\"({}) at index {}", self.current().unwrap() as char, self.current().unwrap(), self.index);
        }
        debug!("Found a colon at index {}", self.index);
        self.index += 1;
        debug!("(Skip colon) Shifted index to {}", self.index);
    }

    fn find_key(&mut self) -> String {

        debug!("Searching key starting at index {}", self.index);

        let mut quote_char: u8 = 0x00;
        let mut index: usize = self.index;
        for c in self.slice() { 
            index += 1;
            if QUOTE_CHARS.contains(&c) {
                quote_char = *c;
                break;
            }
        }
        self.index = index;
    
    
        let mut key:Vec<u8>  = Vec::new();
        let mut escaped: bool = false;
    
        for c in self.slice() {
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
        self.index = index;

  
    
        return String::from_utf8(key).unwrap();
    }
}


 

impl Iterator for JsonBytesWrapper {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.content[self.index];
        self.index += 1;
        return Some(c);
    }
}