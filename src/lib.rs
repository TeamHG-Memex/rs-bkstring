#[macro_use] extern crate cpython;
extern crate pyo3;

// use pyo3::*;

pub mod bktree;
pub mod bknode;
pub mod dist;
pub mod pyutil;

use cpython::{PyObject, PyResult, PyType, Python, PyErr, PyList, PyString, PyLong, PyInt, PyTuple, ToPyObject, PythonObject, ObjectProtocol};

use std::cell;
use bktree::{Dist, PyBkTree};

fn py_word_to_vec(py: Python, word: PyObject) -> Vec<char> {
    return word.extract::<String>(py).unwrap().chars().collect::<Vec<char>>();
}

fn vec_to_py_string(py: Python, word: &Vec<char>) -> PyObject {
    return word.iter().cloned().collect::<String>().to_py_object(py).into_object();
}

py_class!(class BkTree |py| {
    data tree: cell::RefCell<bktree::PyBkTree>;

    def __new__(_cls, func: Option<PyObject>) -> PyResult<BkTree> {
        match func {
            Some(func) => {
                BkTree::create_instance(py, cell::RefCell::new(bktree::PyBkTree::new(Some(func))))
            },
            None => {
                BkTree::create_instance(py, cell::RefCell::new(bktree::PyBkTree::new(None)))
            }
        }
    }

    def add(&self, word: PyObject) -> PyResult<PyObject> {
        self.tree(py).borrow_mut().add(word);

        Ok(py.None())
    }

    def add_list(&self, list: PyList) -> PyResult<PyObject> {
        self.tree(py).borrow_mut().add_list(list);

        Ok(py.None())
    }

    def search(&self, word: PyObject, dist: f64) -> PyResult<PyList> {
        let results = self.tree(py).borrow().search(word, dist);

        Ok(PyList::new(py, &results[..]))
    }
});

fn get_type(py: Python, arg: PyObject) -> PyResult<PyString> {
    let py_type = arg.get_type(py);
    let name = py_type.name(py);

    return Ok(name.to_py_object(py));
}

#[py::modinit(bktree)]
py_module_initializer!(bktree, initbktree, PyInit_bktree, |py, m| {
    m.add(py, "__doc__", "A BK Tree library written in Rust with Python bindings.")?;
    m.add_class::<BkTree>(py)?;

    Ok(())
});
