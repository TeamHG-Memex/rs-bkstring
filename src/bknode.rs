use std::default::Default;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct _BkNode<T> {
    pub word: Vec<T>,
    pub dist: usize,
    pub children: Vec<_BkNode<T>>,
}

impl<T> PartialEq for _BkNode<T> {
    fn eq(&self, other: &_BkNode<T>) -> bool {
        self.dist == other.dist
    }
}

impl<T> Eq for _BkNode<T> {}

impl<T> Ord for _BkNode<T> {
    fn cmp(&self, other: &_BkNode<T>) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl<T> PartialOrd for _BkNode<T> {
    fn partial_cmp(&self, other: &_BkNode<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Sized> _BkNode<T> {
    pub fn add(&mut self, word: Vec<T>, dist: fn(Vec<T>, Vec<T>) -> usize) {
        // Handle the initial node case.
        if self.word.is_empty() {
            self.word = word;
            return;
        }

        let curr_dist = dist(self.word.to_owned(), word.to_owned());

        // Binary search returns the index of the search value, or the index where, if the value is inserted, it will retain ordering.
        match self.children.binary_search_by(|curr_node| curr_node.dist.cmp(&curr_dist)) {
            Ok(child_idx) => {
                self.children[child_idx].add(word.to_owned(), dist);
            },
            Err(insert_idx) => {
                let node = _BkNode {
                    word: word.to_owned(),
                    dist: curr_dist,
                    children: vec![]
                };

                self.children.insert(insert_idx, node);
            }
        };
    }
}

impl<T> Default for _BkNode<T> {
    fn default() -> _BkNode<T> {
        _BkNode {
            word: vec![],
            dist: 0,
            children: vec![]
        }
    }
}

pub type BkNode<T> = _BkNode<T>;
