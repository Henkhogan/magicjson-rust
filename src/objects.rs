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

#[derive(Clone, Debug)]
pub struct JsonCustomType {
    pub name: String,
    pub value: String,
}

impl IntoPy<Py<PyAny>> for JsonCustomType {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        return PyTuple::new(py, vec!(self.name,self.value)).into();
    }
}

//#[pyclass(module="magicjson")]
#[derive(Clone, Debug)]
//#[derive(Clone, Debug)]
pub enum JsonItem {
    Bool(bool),
    Dict(HashMap<JsonKey, JsonItem>),
    Int(i32),
    List(Vec<JsonItem>),
    Float(f64),
    Null(), 
    Str(String),
    Custom(JsonCustomType),
    
}

//#[pymethods]
impl IntoPy<Py<PyAny>> for JsonItem {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            JsonItem::Bool(_value) => {
                return PyBool::new(py, _value).into();
            },
            JsonItem::Dict(_value) => {
                return _value.into_py(py);
            },
            JsonItem::Int(_value) => {
                return _value.into_py(py);
            },
            JsonItem::List(_value) => {
                return PyList::new(py, _value.into_iter().map(|i|i.into_py(py))).into();
            },
            JsonItem::Float(_value) => {
                return PyFloat::new(py, _value).into();
            },
            JsonItem::Null() => {
                return ().into_py(py);
            },
            JsonItem::Str(_value) => {
                return PyString::new(py, &_value).into();
            },
            JsonItem::Custom(_value) => {
                return _value.into_py(py);
            },
        }

    }
}
