use crate::trie::Node;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Given a file path, load a dictionary into a [trie](./trie.rs)
pub fn dict(path: &str) -> Node<char> {
    let mut trie = Node::new();
    let f = File::open(path).expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        if let Ok(line) = line {
            trie.insert(&mut line.chars());
        }
    }
    trie
}
