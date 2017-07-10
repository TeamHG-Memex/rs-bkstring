use cpython::*;

use std::collections::HashMap;

#[derive(Clone)]
pub enum ConversionType {
    I(i32),
    S(String),
    N(Option<bool>)
}

impl ToPyObject for ConversionType {
    type ObjectType = PyObject;

    fn to_py_object(&self, py: Python) -> PyObject {
        match *self {
            ConversionType::I(ref value) => {
                value.to_py_object(py).into_object()
            },
            ConversionType::S(ref value) => {
                value.to_py_object(py).into_object()
            },
            ConversionType::N(ref value) => {
                py.None().into_object()
            }
        }
    }

    fn into_py_object(self, py: Python) -> PyObject {
        match self {
            ConversionType::I(value) => {
                value.to_py_object(py).into_object()
            },
            ConversionType::S(value) => {
                value.to_py_object(py).into_object()
            },
            ConversionType::N(value) => {
                py.None().into_object()
            }
        }
    }
}

pub fn py_word_to_vec(py: Python, word: PyObject) -> Vec<char> {
    return word.extract::<String>(py).unwrap().chars().collect::<Vec<char>>();
}

pub fn vec_to_py_string(py: Python, word: &Vec<char>) -> PyObject {
    return word.iter().cloned().collect::<String>().to_py_object(py).into_object();
}

pub fn use_function(func: &PyObject, arg1: PyObject, arg2: PyObject) -> f64 {
    let gil = Python::acquire_gil();
    let py = gil.python();

    // let py_arg1 = rust_to_py(py, arg1);
    // let py_arg2 = rust_to_py(py, arg2);


    // let py_arg1 = vec_to_py_string(py, &arg1);
    // let py_arg2 = vec_to_py_string(py, &arg2);

    return func.call(py, PyTuple::new(py, &[arg1, arg2]), None).unwrap().extract::<f64>(py).unwrap();
}

fn from_string(py: Python, arg: PyObject) -> ConversionType {
    return ConversionType::S(arg.extract::<String>(py).unwrap());
}

fn from_int(py: Python, arg: PyObject) -> ConversionType {
    return ConversionType::I(arg.extract::<i32>(py).unwrap());
}

fn to_none(_py: Python, _arg: PyObject) -> ConversionType {
    return ConversionType::N(None);
}

pub fn py_to_rust(py: Python, arg: PyObject) -> ConversionType {
    let py_type = arg.get_type(py);
    let name = py_type.name(py);

    let convert = match name.as_ref() {
        "str" => from_string,
        "int" => from_int,
        _ => to_none,
    };

    return convert(py, arg);
}

pub fn rust_to_py<T: ToPyObject>(py:Python, arg: Vec<T>) -> PyObject {
    let mut ret: Vec<PyObject> = vec![];

    for item in &arg {
        ret.push(item.clone().to_py_object(py).into_object());
    }

    return PyList::new(py, ret.as_slice()).into_object();
}

pub fn py_list_to_vec(py: Python, list: PyList) -> Vec<PyObject> {
    let mut ret = vec![];

    for value in list.iter(py) {
        ret.push(value);
    }

    return ret;
}

// pub fn py_list_to_rust(py: Python, arg: PyObject) -> ConversionType {
//     return ConversionType::V(vec![]);
// }
