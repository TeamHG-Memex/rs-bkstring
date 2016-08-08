#![feature(test)]
// #![feature(libc)]

// #![feature(plugin)]
// #![plugin(interpolate_idents)]

// #[macro_use] extern crate cpython;

mod bktree;
mod bknode;
mod bkgraph;
mod bkdist;

use bktree::*;
use bknode::*;
use bkgraph::*;
use bkdist::*;

extern crate rand;
extern crate test;
// extern crate libc;
// use std::ffi::CStr;
// use std::os::raw::c_char;

// use cpython::{PyResult, Python, PyList, PyString, PyObject};
//
// py_module_initializer!(bkstring, initbkstring, PyInit_bkstring, |py, m| {
//     try!(m.add(py, "__doc__", "A test module for effing around with."));
//     try!(m.add(py, "BkTree", py.get_type::<BkTree>()));
//     Ok(())
// });
//
// #[no_mangle]
// pub extern fn c_char_to_string(c_ptr: *const c_char) -> String {
//     let bytes = unsafe {
//         CStr::from_ptr(c_ptr).to_bytes()
//     };
//
//     let mut byte_vec: Vec<u8> = vec![];
//     for c in bytes {
//         byte_vec.push(c.to_owned());
//     }
//
//     return match String::from_utf8(byte_vec) {
//         Ok(s) => s,
//         Err(err) => panic!("{}", err)
//     };
// }

// fn new_bktree(_py: Python, func: String) -> PyResult<PyRustType<BkTree>> {
//     PyRustTypeBuilder::<BkTree>::new(_py, "BkTree")
//         .add("tree", PyRustObject::<BkTree>)
//         .add("add", py_method!(add(word: String)))
//         .add("search", py_method!(search(word: String, dist: u64)))
//         .finish()
// }
//
// fn add(_py: Python, slf: &PyRustObject<BkTree>, word: String) -> PyResult<u64> {
//     Ok(0)
// }
//
// fn search(_py: Python, slf: &PyRustObject<BkTree>, word: String, dist: u64) -> PyResult<PyList> {
//     let list = PyList::new(_py, &[]);
//     Ok(list)
// }

// py_class!(class PyBkTree |py| {
//     def __new__(_cls) -> PyResult<PyBkTree> {
//         Ok(BkTree::new(Some(Metric::l_dist())))
//     }
// });

// #[no_mangle]
// pub extern fn add_to_bktree(tree: *const BkTree, string: *const c_char) -> *const BkTree {
//     let mut new_tree = unsafe { &*tree };
//     println!("new tree after unsafe!");
//     let the_string = c_char_to_string(string);
//     println!("yay; stuff!");
//     new_tree.to_owned();
//     // .add(the_string);
//     return new_tree as *const BkTree;
// }

// #[no_mangle]
// pub extern fn l_dist() -> Metric {
//     return Metric::l_dist();
// }
