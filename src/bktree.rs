use std::default::Default;
use std::f64::MIN_POSITIVE;

use bknode::{BkNode, PyBkNode};
use dist::*;
use pyutil::{use_function};

use cpython::{Python, PyClone, PyObject, PyList};

pub type Dist<T> = fn(Vec<T>, Vec<T>) -> usize;

pub struct _BkTree<T> {
    _root: BkNode<T>,
    dist: Dist<T>
}

impl<T: Eq + Clone + Sized> _BkTree<T> {
    pub extern fn new(func: Option<Dist<T>>) -> _BkTree<T> {
        match func {
            Some(func) => _BkTree {
                _root: Default::default(),
                dist: func
            },
            None => _BkTree {
                _root: Default::default(),
                dist: levenshtein_dist
            }
        }
    }

    pub extern fn add(&mut self, word: Vec<T>) {
        self._root.add(word, self.dist);
    }

    pub extern fn add_list(&mut self, list: Vec<Vec<T>>) {
        for word in list {
            self.add(word);
        }
    }

    fn r_search(&self, node: &BkNode<T>, word: Vec<T>, dist: usize, s_list: &mut Vec<Vec<T>>) {
        let curr_word = &node.word;
        let curr_dist = (self.dist)(curr_word.to_owned(), word.to_owned());

        let min_dist = match curr_dist > dist {
            true => curr_dist - dist,
            false => {
                s_list.push(curr_word.to_owned());
                0
            }
        };

        let max_dist = curr_dist + dist + 1;

        let min_idx = match node.children.binary_search_by(|probe| probe.dist.cmp(&min_dist)) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        let max_idx = match node.children.binary_search_by(|probe| probe.dist.cmp(&max_dist)) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        for i in min_idx..max_idx {
            self.r_search(&node.children[i], word.to_owned(), dist, s_list);
        }
    }

    pub extern fn search(&self, word: Vec<T>, dist: usize) -> Vec<Vec<T>> {
        let mut results: Vec<Vec<T>> = vec![];

        self.r_search(&self._root, word.clone(), dist, &mut results);
        return results;
    }
}

pub type BkTree<T> = _BkTree<T>;

pub struct _PyBkTree {
    _root: PyBkNode,
    dist: PyObject
}

impl _PyBkTree {
    pub extern fn new(func: Option<PyObject>) -> _PyBkTree {
        match func {
            Some(func) => _PyBkTree {
                _root: Default::default(),
                dist: func
            },
            None => {
                let gil = Python::acquire_gil();
                let py = gil.python();

                _PyBkTree {
                    _root: Default::default(),
                    dist: py.None()
                }
            }
        }
    }

    pub extern fn add(&mut self, word: PyObject) {
        self._root.add(word, &self.dist);
    }

    pub extern fn add_list(&mut self, list: PyList) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        for word in list.iter(py) {
            self.add(word);
        }
    }

    fn r_search(&self, node: &PyBkNode, word: PyObject, dist: f64, s_list: &mut Vec<PyObject>) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let curr_word = node.word.clone_ref(py);
        let curr_dist = use_function(&self.dist, curr_word.clone_ref(py), word.clone_ref(py));

        let min_dist = match curr_dist > dist {
            true => curr_dist - dist,
            false => {
                s_list.push(curr_word.clone_ref(py));
                0.0
            }
        };

        let max_dist = curr_dist + dist + MIN_POSITIVE;

        let min_idx = match node.children.binary_search_by(|probe| probe.dist.partial_cmp(&min_dist).unwrap()) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        let max_idx = match node.children.binary_search_by(|probe| probe.dist.partial_cmp(&max_dist).unwrap()) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        for i in min_idx..max_idx {
            self.r_search(&node.children[i], word.clone_ref(py), dist, s_list);
        }
    }

    pub extern fn search(&self, word: PyObject, dist: f64) -> Vec<PyObject> {
        let mut results: Vec<PyObject> = vec![];

        self.r_search(&self._root, word, dist, &mut results);
        return results;
    }
}

pub type PyBkTree = _PyBkTree;

#[test]
fn add_list_test() {
    let mut b: BkTree<char> = BkTree::new(None);
    let list = vec!["foo".chars().collect(), "bar".chars().collect(), "baz".chars().collect()];

    b.add_list(list);
    let test1: Vec<char> = "foo".chars().collect();
    let test2: Vec<char> = "bar".chars().collect();
    let test3: Vec<char> = "baz".chars().collect();

    assert_eq!(b._root.word, test1);
    assert_eq!(b._root.children[0].word, test2);
    assert_eq!(b._root.children[0].children[0].word, test3);
}

#[test]
fn search_test() {
    let mut b: BkTree<char> = BkTree::new(None);

    b.add("foo".chars().collect());
    b.add("food".chars().collect());
    b.add("foodb".chars().collect());
    b.add("foodc".chars().collect());
    b.add("foodd".chars().collect());
    b.add("foe".chars().collect());
    b.add("fooda".chars().collect());

    {
        let list = b.search("foo".chars().collect(), 0);

        assert!(list.contains(&"foo".chars().collect()));
        assert!(!list.contains(&"food".chars().collect()));
        assert!(!list.contains(&"fooda".chars().collect()));
        assert!(!list.contains(&"foe".chars().collect()));
    }

    {
        let list = b.search("foo".chars().collect(), 1);

        assert!(list.contains(&"foo".chars().collect()));
        assert!(list.contains(&"foe".chars().collect()));
        assert!(!list.contains(&"fooda".chars().collect()));
    }

    {
        let list = b.search("foo".chars().collect(), 2);

        assert!(list.contains(&"foo".chars().collect()));
        assert!(list.contains(&"fooda".chars().collect()));
        assert!(list.contains(&"foodb".chars().collect()));
        assert!(list.contains(&"foodc".chars().collect()));
        assert!(list.contains(&"foodd".chars().collect()));
        assert!(list.contains(&"foe".chars().collect()));
    }

    {
        let list = b.search("bar".chars().collect(), 1);
        println!("{:?}", list);

        assert!(list.is_empty());
    }
}

#[test]
fn default_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(None);

    let test1: Vec<char> = "foo".chars().collect();
    b.add(test1.to_owned());
    assert_eq!(b._root.word, test1.to_owned());

    let test2: Vec<char> = "bar".chars().collect();
    b.add(test2.to_owned());
    assert_eq!(b._root.children[0].word, test2.to_owned());
}

#[test]
fn jaccard_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(Some(jaccard_dist));

    let test1: Vec<char> = "foo".chars().collect();

    b.add(test1.to_owned());
    assert_eq!(b._root.word, test1.to_owned());

    let test2: Vec<char> = "bar".chars().collect();

    b.add(test2.to_owned());
    assert_eq!(b._root.children[0].word, test2.to_owned());
}

#[test]
fn modified_jaccard_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(Some(modified_jaccard_dist));

    let test1: Vec<char> = "foo".chars().collect();

    b.add(test1.to_owned());
    assert_eq!(b._root.word, test1.to_owned());

    let test2: Vec<char> = "bar".chars().collect();

    b.add(test2.to_owned());
    assert_eq!(b._root.children[0].word, test2.to_owned());
}

#[test]
fn hamming_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(Some(hamming_dist));

    let test1: Vec<char> = "0".chars().collect();

    b.add(test1.to_owned());
    assert_eq!(b._root.word, test1.to_owned());

    let test2: Vec<char> = "f".chars().collect();

    b.add(test2.to_owned());
    assert_eq!(b._root.children[0].word, test2.to_owned());
}
