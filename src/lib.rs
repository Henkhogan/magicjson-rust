use pyo3::prelude::*;
use std::io::Read;
use std::fs;

mod wrapper;
use wrapper::{JsonBytesWrapper, JsonWrapperTrait};

mod objects;
use objects::{JsonItem, JsonKey};

mod handler;
use handler::handle_dict_or_list;

mod constants;

#[macro_use]
extern crate lazy_static;



/// Formats the sum of two numbers as string.
#[pyfunction]
fn load_file(file_path: String) -> JsonItem {

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
    
    //Builder::new()
    //.format(|buf, record| {
    //    writeln!(buf,
    //        "{} [{}] - {}",
    //        Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //        record.level(),
    //        record.args()
    //    )
    //})
    //.filter(None, LevelFilter::Debug)
    //.init();

    pyo3_log::init();



    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    Ok(())
}
