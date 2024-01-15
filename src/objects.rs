use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDateTime, PyDict, PyInt, PyList, PyFloat, PyString, PyTuple, PyCode, PyAny, PyTzInfo};
use std::collections::HashMap;
use iso8601::{Date, DateTime as IsoDateTime};

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
    Datetime(IsoDateTime),
    Timestamp(f64),
    
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
                return PyTuple::new(py, _value.into_iter().map(|i|i.into_py(py))).into();
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
            JsonItem::Datetime(_value) => {
                match _value.date {
                    Date::YMD { year, month, day } => {
                        return PyDateTime::new(
                            py,
                            year, month as u8, day as u8, 
                            _value.time.hour as u8, _value.time.minute as u8, _value.time.second as u8, _value.time.millisecond*1000, None // ToDo: handle TZ PyTzInfo::from(py, _value.time.tz_offset_hours*60*60+_value.time.tz_offset_minutes*60, false, None)
                        ).unwrap().into();
                    },
                    _ => {
                        panic!("Unsupported date format");
                    }
                }
            },
            JsonItem::Timestamp(_value) => {
                return PyDateTime::from_timestamp(py, _value, None).unwrap().into();
            },
            JsonItem::Custom(_value) => {
                return _value.into_py(py);
            },
        }

    }
}
