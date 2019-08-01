use std::cell::RefCell;
use std::collections::HashMap;

pub struct Node {
    word: bool,
    suff: HashMap<char, Box<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            word: false,
            suff: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &mut Iterator<Item = char>) {
        let c = match word.next() {
            None => {
                self.word = true;
                return;
            }
            Some(c) => c,
        };

        match self.suff.get(&c) {
            None => {
                let mut newtrie = Node::new();
                newtrie.insert(word);
                self.suff.insert(c, Box::new(RefCell::new(newtrie)));
            }
            Some(node) => {
                node.borrow_mut().insert(word);
            }
        };
    }

    fn search(&self, word: &mut Iterator<Item = char>, endstate: &Fn(&Node) -> bool) -> bool {
        let c = match word.next() {
            None => return endstate(self),
            Some(c) => c,
        };

        match self.suff.get(&c) {
            None => false,
            Some(n) => n.borrow().search(word, endstate),
        }
    }

    pub fn find(&self, word: &mut Iterator<Item = char>) -> bool {
        self.search(word, &|s| s.word)
    }

    pub fn pref(&self, word: &mut Iterator<Item = char>) -> bool {
        self.search(word, &|_s| true)
    }
}

#[cfg(test)]
mod tests {
    use crate::dict::dict;

    #[test]
    fn test_tries() {
        let trie = dict("/usr/share/dict/words");
        assert!(trie.find(&mut "question".to_string().chars()));
        assert!(trie.find(&mut "zigzag".to_string().chars()));
        assert!(!trie.find(&mut "felgercarb".to_string().chars()));
        assert!(!trie.find(&mut "shazbat".to_string().chars()));
        assert!(!trie.find(&mut "oriente".to_string().chars()));
        assert!(trie.pref(&mut "oriente".to_string().chars()));
    }
}
