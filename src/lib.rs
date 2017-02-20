#[macro_use] extern crate cpython;

pub mod bktree;
pub mod bknode;
pub mod dist;

use cpython::{PyObject, PyResult, Python, PyList, ToPyObject, PythonObject};

use std::cell;

py_module_initializer!(bktree, initbktree, PyInit_bktree, |py, m| {
    m.add(py, "__doc__", "A BK Tree library written in Rust with Python bindings.")?;
    m.add_class::<BkTree>(py)?;
    Ok(())
});

fn py_word_to_vec(py: Python, word: PyObject) -> Vec<char> {
    return word.extract::<String>(py).unwrap().chars().collect::<Vec<char>>();
}

fn vec_to_py_string(py: Python, word: &Vec<char>) -> PyObject {
    return word.iter().cloned().collect::<String>().to_py_object(py).into_object();
}

py_class!(class BkTree |py| {
    data tree: cell::RefCell<bktree::BkTree<char>>;

    def __new__(_cls) -> PyResult<BkTree> {
        BkTree::create_instance(py, cell::RefCell::new(bktree::BkTree::new(None)))
    }

    def add(&self, word: PyObject) -> PyResult<PyObject> {
        let word_vec: Vec<char> = py_word_to_vec(py, word);
        self.tree(py).borrow_mut().add(word_vec);

        Ok(py.None())
    }

    def add_list(&self, list: PyList) -> PyResult<PyObject> {
        let words: Vec<Vec<char>> = list.iter(py).map(|w| { py_word_to_vec(py, w) }).collect();

        self.tree(py).borrow_mut().add_list(words);

        Ok(py.None())
    }

    def search(&self, word: PyObject, dist: usize) -> PyResult<PyList> {
        let search_word = py_word_to_vec(py, word);
        let results = self.tree(py).borrow().search(search_word, dist);

        let list: Vec<PyObject> = results.iter().map(|w| {vec_to_py_string(py, w)}).collect();

        Ok(PyList::new(py, &list[..]))
    }
});
