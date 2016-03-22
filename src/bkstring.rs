use std::default::Default;
use std::cmp::min;

use bknode::BkNode;
use bkdist::l_dist;

use rand::{thread_rng, Rng};
use test::Bencher;

type Metric = fn(String, String) -> usize;

pub struct _BkTree {
    _root: BkNode,
    dist: Metric
}

impl _BkTree {
    pub fn new(func: Option<fn(String, String) -> usize>) -> _BkTree {
        match func {
            Some(func) => _BkTree {
                _root: Default::default(),
                dist: func
            },
            None => _BkTree {
                _root: Default::default(),
                dist: l_dist
            }
        }
    }

    pub fn add(&mut self, word: String) {
        self._root.add(word, self.dist);
    }

    pub fn add_list(&mut self, list: Vec<String>) {
        for word in list {
            self.add(word);
        }
    }

    fn r_search(&self, node: &BkNode, word: String, dist: usize, s_list: &mut Vec<String>) {
        match node.word {
            Some(ref curr_word) => {
                let curr_dist = (self.dist)(curr_word.clone(), word.clone());
                let min_dist = {
                    if curr_dist <= dist {
                        s_list.push(curr_word.clone());
                        0
                    } else {
                        curr_dist - dist
                    }
                };

                let max_dist = min(curr_dist + dist + 1, node.children.len());

                for i in min_dist..max_dist {
                    self.r_search(&node.children[i], word.clone(), dist, s_list);
                }
            },
            None => {
                return;
            }
        }
    }

    pub fn search(&self, word: String, dist: usize) -> Vec<String> {
        let mut s_list: Vec<String> = vec![];

        self.r_search(&self._root, word.clone(), dist, &mut s_list);
        return s_list;
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
fn bench_default(b: &mut Bencher) {
    let len = 30000;
    let mut bk = BkTree::new(None);
    let mut names: Vec<String> = vec![];

    for i in 0..len {
        let s = thread_rng()
            .gen_ascii_chars()
            .take(10)
            .collect::<String>();
        names.push(s.clone());
    }

    for i in 0..names.len() {
        bk.add(names[i].clone());
    }
    b.iter(|| {
        bk.search(names[0].clone(), 2);
    });
    // b.iter(|| {
    // });
}
