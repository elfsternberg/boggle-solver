#![deny(missing_docs)]

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A helper function to load a dictionary into memory.

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
