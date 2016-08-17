#[macro_use] extern crate cpython;

mod bkdist;
mod bknode;
mod bktree;

use cpython::{PyResult, Python, PyList, PyString, PyObject, PythonObject};
use std::{cell};

py_module_initializer!(libbkstring, initlibbkstring, PyInit_libbkstring, |py, m| {
    try!(m.add(py, "__doc__", "A BK Tree library written in Rust with Python bindings."));
    try!(m.add_class::<BkTree>(py));
    Ok(())
});

py_class!(class BkTree |py| {
    data tree: cell::RefCell<bktree::BkTree>;

    def __new__(_cls) -> PyResult<BkTree> {
        BkTree::create_instance(py, cell::RefCell::new(bktree::BkTree::new(None)))
    }

    def add(&self, word: String) -> PyResult<PyObject> {
        self.tree(py).borrow_mut().add(word);
        Ok(Python::None(py))
    }

    def add_list(&self, list: PyList) -> PyResult<PyObject> {
        let words = list.iter(py).map(|w| w.extract(py).unwrap()).collect::<Vec<String>>();
        self.tree(py).borrow_mut().add_list(words);
        Ok(Python::None(py))
    }

    def search(&self, word: String, dist: usize) -> PyResult<PyList> {
        let results: Vec<String> = self.tree(py).borrow_mut().search(word, dist);
        let list = results.iter().map(|s| PyString::new(py, s).into_object()).collect::<Vec<_>>();
        Ok(PyList::new(py, &list[..]))
    }
});
