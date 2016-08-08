use bktree::BkTree;
use bkdist::Metric;

use std::collections::HashMap;

type Diff = fn(usize, usize, usize) -> usize;
type Len = fn(String) -> usize;

pub struct _BkGraph {
    trees: HashMap<usize, BkTree>,
    metric: Metric,
    diff: Diff,
    len: Len
}

impl _BkGraph {
    pub extern fn new(metric: Option<Metric>) -> _BkGraph {
        match metric {
            Some(metric) => _BkGraph {
                trees: HashMap::new(),
                metric: metric.clone(),
                diff: metric.diff,
                len: metric.len
            },
            None => _BkGraph {
                trees: HashMap::new(),
                metric: Metric::l_dist(),
                diff: Metric::l_dist().diff,
                len: Metric::l_dist().len
            }
        }
    }

    pub extern fn add(&mut self, word: String) {
        let len = (self.len)(word.clone());

        let mut tree = self.trees.entry(len).or_insert(BkTree::new(Some(self.metric.clone())));
        tree.add(word.clone());
    }

    pub extern fn add_list(&mut self, list: Vec<String>) {
        for word in list {
            self.add(word);
        }
    }

    pub extern fn search(&self, word: String, dist: usize) -> Vec<String> {
        let len = (self.len)(word.clone());
        let mut results: Vec<String> = vec![];

        for key in self.trees.keys() {
            let diff = (self.diff)(len, key.clone(), dist);
            results.append(&mut self.trees[key].search(word.clone(), diff));
        }

        return results;
    }

    pub extern fn variance_search(&self, word: String, dist: usize, variance: usize) -> Vec<String> {
        let len = word.chars().count();
        let mut results: Vec<String> = vec![];

        for key in self.trees.keys() {
            if len as i64 - variance as i64 > len as i64 - (key.clone() as i64 - len as i64).abs() {
                continue;
            }

            if len + variance < len + (key.clone() as i64 - len as i64).abs() as usize {
                continue;
            }

            let diff = (self.diff) (len, key.clone(), dist);
            results.append(&mut self.trees[key].search(word.clone(), diff));
        }

        return results;
    }
}

pub type BkGraph = _BkGraph;

#[test]
fn test_add() {
    let mut b = BkGraph::new(None);
    b.add("foo".to_string());
}

#[test]
fn test_all() {
    let mut b = BkGraph::new(None);
    b.add("foo".to_string());
    assert_eq!(b.search("foo".to_string(), 0)[0], "foo".to_string());
}

#[test]
fn test_add_list() {
    let mut b = BkGraph::new(None);
    let list = vec!["foo".to_string(), "bar".to_string()];
    b.add_list(list);
}

#[test]
fn test_var_search() {
    let mut b = BkGraph::new(None);
    b.add("foo".to_string());
    assert!(!b.variance_search("f".to_string(), 3, 1).contains(&"foo".to_string()));
    assert!(b.variance_search("f".to_string(), 0, 3).contains(&"foo".to_string()));
}
