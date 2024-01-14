use pyo3::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::{fmt, hash::Hash};



#[pyclass(module="magicjson")]
#[derive(Clone, Copy, Debug, Hash)]
pub enum JsonType {
    Null,
    List,
    Dict,
    String,
    Int,
    Float,
    Bool,
    CustomType,
}

#[pymethods]
impl JsonType {
    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
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
pub struct JsonItem {
    pub key: Option<String>,
    // value should be converted to bytearray but is currently a list of bytes
    pub value_bool: Option<bool>,
    pub value_dict: Option<HashMap<String, JsonItem>>,
    pub value_int: Option<i8>,
    pub value_list: Option<Vec<JsonItem>>,
    pub value_float: Option<f32>,
    pub value_str: Option<String>,
    pub items: Option<Vec<JsonItem>>,
    pub value_type: JsonType,
    pub value_custom_type: Option<String>,
}

#[pymethods]
impl JsonItem {

    
}

//impl IntoPy<PyObject> for JsonItem {
//    fn into_py(self, py: Python<'_>) -> PyObject {
//        return PyObject {
//            key: self.key.into_py(py);
//            let value = self.value.into_py(py);
//            let items = self.items.into_py(py);
//            let value_type = self.value_type.into_py(py);
//            let value_custom_type = self.value_custom_type.into_py(py);
//
//        }
//        //self.top_level_type.into_py(py),
//        //self.children.unwrap().into_py(py)
//    }
//}
