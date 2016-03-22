use std::default::Default;

const CHILD_NODES: usize = 256;

#[derive(Clone)]
pub struct _BkNode {
    pub word: Option<String>,
    pub children: Vec<_BkNode>,
}

impl _BkNode {
    pub fn add(&mut self, word: String, dist: fn(String, String) -> usize) {
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

impl Default for _BkNode {
    fn default() -> _BkNode {
        _BkNode {
            children: vec![],
            word: None
        }
    }
}

pub type BkNode = _BkNode;
