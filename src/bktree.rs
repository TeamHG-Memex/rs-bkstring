use std::default::Default;
use std::cmp::min;

use bknode::BkNode;
use bkdist::Metric;

use rand::{thread_rng, Rng};
use test::Bencher;

pub type Dist = fn(String, String) -> usize;

#[derive(Clone)]
pub struct _BkTree {
    _root: BkNode,
    dist: Dist
}

impl _BkTree {
    pub extern fn new(func: Option<Metric>) -> _BkTree {
        match func {
            Some(func) => _BkTree {
                _root: Default::default(),
                dist: func.function
            },
            None => _BkTree {
                _root: Default::default(),
                dist: Metric::l_dist().function
            }
        }
    }

    pub extern fn add(&mut self, word: String) {
        self._root.add(word, self.dist);
    }

    pub extern fn add_list(&mut self, list: Vec<String>) {
        for word in list {
            self.add(word);
        }
    }

    fn r_search(&self, node: &BkNode, word: String, dist: usize, s_list: &mut Vec<String>) {
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

    pub extern fn search(&self, word: String, dist: usize) -> Vec<String> {
        let mut results: Vec<String> = vec![];

        self.r_search(&self._root, word.clone(), dist, &mut results);
        return results;
    }
}

pub type BkTree = _BkTree;

#[test]
fn add_test() {
    let mut b: BkTree = BkTree::new(None);

    b.add("foo".to_string());
    assert_eq!(b._root.word, Some("foo".to_string()));

    b.add("bar".to_string());
    assert_eq!(b._root.children[3].word, Some("bar".to_string()));
}

#[test]
fn add_list_test() {
    let mut b: BkTree = BkTree::new(None);
    let list = vec!["foo".to_string(), "bar".to_string()];

    b.add_list(list);
    assert_eq!(b._root.children[3].word, Some("bar".to_string()));
}

#[test]
fn search_test() {
    let mut b: BkTree = BkTree::new(None);

    b.add("foo".to_string());
    b.add("food".to_string());
    b.add("foe".to_string());

    {
        let list = b.search("foo".to_string(), 0);

        assert!(list.contains(&"foo".to_string()));
    }

    {
        let list = b.search("foo".to_string(), 1);

        assert!(list.contains(&"foo".to_string()));
        assert!(list.contains(&"food".to_string()));
        assert!(list.contains(&"foe".to_string()));
    }

    {
        let list = b.search("bar".to_string(), 1);

        assert!(list.is_empty());
    }
}

#[bench]
fn bench_add(b: &mut Bencher) {
    let len = 1000;
    let mut bk = BkTree::new(None);
    let mut names: Vec<String> = vec![];

    let mut i = 0;

    while i < len {
        let s = thread_rng()
            .gen_ascii_chars()
            .take(6)
            .collect::<String>();
        names.push(s.clone());

        i += 1;
    }

    b.iter(|| {
        bk.add_list(names.to_owned());
    });
}

#[bench]
fn bench_default(b: &mut Bencher) {
    let len = 1000;
    let mut bk = BkTree::new(None);
    let mut names: Vec<String> = vec![];

    let mut i = 0;

    while i < len {
        let s = thread_rng()
            .gen_ascii_chars()
            .take(6)
            .collect::<String>();
        names.push(s.clone());

        i += 1;
    }

    bk.add_list(names.to_owned());

    b.iter(|| {
        bk.search(names[0].clone(), 2);
    });
}
