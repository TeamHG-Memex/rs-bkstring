use std::default::Default;

const CHILD_NODES: usize = 256;

#[derive(Clone)]
pub struct _BkNode<T> {
    pub word: Option<Vec<T>>,
    pub children: Vec<_BkNode<T>>,
}

impl<T: Clone + Sized> _BkNode<T> {
    pub fn add(&mut self, word: Vec<T>, dist: fn(Vec<T>, Vec<T>) -> usize) {
        match self.word {
            Some(ref node_word) => {
                let d = dist(node_word.to_owned(), word.to_owned());

                if d == 0 {
                    return;
                }

                self.children[d].add(word, dist);
            },
            None => {
                self.word = Some(word);
                self.children = vec![_BkNode::default(); CHILD_NODES];
            }
        }
    }

}

impl<T> Default for _BkNode<T> {
    fn default() -> _BkNode<T> {
        _BkNode {
            word: None,
            children: vec![]
        }
    }
}

pub type BkNode<T> = _BkNode<T>;
