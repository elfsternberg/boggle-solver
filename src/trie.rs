// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A generic trie structure.
//!
//! This library implements a fairly generic trie structure in which
//! the edges rather than the nodes are the source of the data, which
//! is more or less what you want for a dictionary trie.  

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

/// A single node in a trie
///
/// The design of this trie has no root object; any node can perform the
/// role of root.  The whole point of this structure is that, if you
/// have a large collection of strings and you want to determine if
/// the string is present in the collection, a trie is a highly
/// time-efficient but space-costly, completely deterministic means of
/// doing that.

pub struct Node<C>
where
    C: Copy + Hash + Eq,
{
    word: bool,
    suff: HashMap<C, Box<RefCell<Node<C>>>>,
}

impl<C> Node<C>
where
    C: Copy + Hash + Eq,
{
    pub fn new() -> Node<C> {
        Node {
            word: false,
            suff: HashMap::new(),
        }
    }

    /// Insert a word into the trie.  This function populates descendent
    /// nodes with the rest of the iterator after processing the
    /// letter given.  If there is no letter, this node is marked as a
    /// terminator ("yes, that is a word") and processing ends.
    pub fn insert(&mut self, word: &mut Iterator<Item = C>) {
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

    /// Search for a word or prefix.  The endstate function determines
    /// which.  If the word passed in is exhausted, we return the
    /// endstate.  So for find(), the endstate is "is the terminator
    /// node a word node?"  But for pref(), which only tells you if
    /// the prefix xists, the endstate is "does this node exist at
    /// all?"
    fn search(&self, word: &mut Iterator<Item = C>, endstate: &Fn(&Node<C>) -> bool) -> bool {
        let c = match word.next() {
            None => return endstate(self),
            Some(c) => c,
        };

        match self.suff.get(&c) {
            None => false,
            Some(n) => n.borrow().search(word, endstate),
        }
    }

    /// Determine if the word is in the trie.
    pub fn find(&self, word: &mut Iterator<Item = C>) -> bool {
        self.search(word, &|s| s.word)
    }

    /// Determine if the word is in the trie, or is the prefix of a word
    /// in the trie.
    pub fn pref(&self, word: &mut Iterator<Item = C>) -> bool {
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
