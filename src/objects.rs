use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyInt, PyList, PyFloat, PyString, PyTuple, PyCode, PyAny};
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

pub type JsonKey = String;

//#[pyclass(module="magicjson")]
#[derive(Clone, Debug)]
//#[derive(Clone, Debug)]
pub enum JsonItem {
    Bool(JsonType, bool),
    Dict(JsonType, HashMap<JsonKey, JsonItem>),
    Int(JsonType, i32),
    List(JsonType, Vec<JsonItem>),
    Float(JsonType, f64),
    Null(JsonType), 
    Str(JsonType, String),
    Custom(JsonType, String, String),
    
}

//#[pymethods]
impl IntoPy<Py<PyAny>> for JsonItem {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            JsonItem::Bool(_json_type, _value) => {
                return PyBool::new(py, _value).into();
            },
            JsonItem::Dict(_json_type, _value) => {
                return _value.into_py(py);
            },
            JsonItem::Int(_json_type, _value) => {
                return _value.into_py(py);
            },
            JsonItem::List(_json_type, _value) => {
                return PyList::new(py, _value.into_iter().map(|i|i.into_py(py))).into();
            },
            JsonItem::Float(_json_type, _value) => {
                return PyFloat::new(py, _value).into();
            },
            JsonItem::Null(_json_type) => {
                return ().into_py(py);
            },
            JsonItem::Str(_json_type, _value) => {
                return PyString::new(py, &_value).into();
            },
            JsonItem::Custom(_json_type, _custom_type, _value) => {
                return PyTuple::new(py, vec!(&_custom_type,& _value)).into();
            },
        }

    }
}
#[pyclass(module="magicjson", get_all)]
#[derive(Clone, Debug)]
pub struct JsonItemOld {
    pub key: Option<String>,
    // value should be converted to bytearray but is currently a list of bytes
    pub value_bool: Option<bool>,
    pub value_dict: Option<HashMap<String, JsonItemOld>>,
    pub value_int: Option<i8>,
    pub value_list: Option<Vec<JsonItemOld>>,
    pub value_float: Option<f32>,
    pub value_str: Option<String>,
    pub items: Option<Vec<JsonItemOld>>,
    pub value_type: JsonType,
    pub value_custom_type: Option<String>
}

trait JsonItemTrait {
    fn __hash__(&self) -> u64;
}

#[pymethods]
impl JsonItemOld {

    
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
