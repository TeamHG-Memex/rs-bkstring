use std::default::Default;
use std::cmp::min;

use bknode::BkNode;
use dist::*;

pub type Dist<T> = fn(Vec<T>, Vec<T>) -> usize;

#[derive(Clone)]
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
        match node.word {
            Some(ref curr_word) => {
                let curr_dist = (self.dist)(curr_word.to_owned(), word.to_owned());
                let min_dist = {
                    if curr_dist <= dist {
                        s_list.push(curr_word.to_owned());
                        0
                    } else {
                        curr_dist - dist
                    }
                };

                let max_dist = min(curr_dist + dist + 1, node.children.len());

                for i in min_dist..max_dist {
                    self.r_search(&node.children[i], word.to_owned(), dist, s_list);
                }
            },
            None => {}
        }
    }

    pub extern fn search(&self, word: Vec<T>, dist: usize) -> Vec<Vec<T>> {
        let mut results: Vec<Vec<T>> = vec![];

        self.r_search(&self._root, word.clone(), dist, &mut results);
        return results;
    }
}

pub type BkTree<T> = _BkTree<T>;

#[test]
fn add_list_test() {
    let mut b: BkTree<char> = BkTree::new(None);
    let list = vec!["foo".chars().collect(), "bar".chars().collect()];

    b.add_list(list);
    assert_eq!(b._root.children[3].word, Some("bar".chars().collect()));
}

#[test]
fn search_test() {
    let mut b: BkTree<char> = BkTree::new(None);

    b.add("foo".chars().collect());
    b.add("food".chars().collect());
    b.add("foe".chars().collect());

    {
        let list = b.search("foo".chars().collect(), 0);

        assert!(list.contains(&"foo".chars().collect()));
    }

    {
        let list = b.search("foo".chars().collect(), 1);

        assert!(list.contains(&"foo".chars().collect()));
        assert!(list.contains(&"food".chars().collect()));
        assert!(list.contains(&"foe".chars().collect()));
    }

    {
        let list = b.search("bar".chars().collect(), 1);

        assert!(list.is_empty());
    }
}

#[test]
fn default_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(None);

    b.add("foo".chars().collect());
    assert_eq!(b._root.word, Some("foo".chars().collect()));

    b.add("bar".chars().collect());
    assert_eq!(b._root.children[3].word, Some("bar".chars().collect()));
}

#[test]
fn jaccard_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(Some(jaccard_dist));
    b.add("foo".chars().collect());
    assert_eq!(b._root.word, Some("foo".chars().collect()));

    b.add("bar".chars().collect());
    assert_eq!(b._root.children[100].word, Some("bar".chars().collect()));
}

#[test]
fn modified_jaccard_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(Some(modified_jaccard_dist));
    b.add("foo".chars().collect());
    assert_eq!(b._root.word, Some("foo".chars().collect()));

    b.add("bar".chars().collect());
    assert_eq!(b._root.children[100].word, Some("bar".chars().collect()));
}

#[test]
fn hamming_dist_add_test() {
    let mut b: BkTree<char> = BkTree::new(Some(hamming_dist));
    b.add("0".chars().collect());
    assert_eq!(b._root.word, Some("0".chars().collect()));

    b.add("f".chars().collect());
    assert_eq!(b._root.children[1].word, Some("f".chars().collect()));
}
