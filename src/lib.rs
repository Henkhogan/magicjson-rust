use std::io::Read;
use std::fs;

mod wrapper;
use wrapper::{JsonBytesWrapper, JsonWrapperTrait};

mod objects;
use objects::{JsonItem, JsonKey, JsonCustomType};

mod handler;
use handler::handle_dict_or_list;

mod constants;

#[macro_use]
extern crate lazy_static;



/// Reads a JSON file and returns a JsonItem
pub fn load_file(file_path: String) -> JsonItem {

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


