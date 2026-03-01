#![deny(missing_docs)]
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Boggle solver
//!
//! The readme has more, but Boggle is a popular game released in 1972
//! in which a collection of 16 dice with letters printed on the sides
//! are tossed into a 4⨯4 grid and then the players have three minutes
//! to find as many valid words as they can (valid according to the
//! dictionary of choice (Americans typically use either Webster's or
//! the Scrabble North American dictionary).

pub mod dict;
pub mod trie;

pub mod board;
pub use board::Board;

extern crate itertools;

#[cfg(feature = "large_board")]
mod large_bitmap;
#[cfg(feature = "large_board")]
mod ledger_large;
#[cfg(feature = "large_board")]
use ledger_large::Ledger;

#[cfg(not(feature = "large_board"))]
mod ledger;
#[cfg(not(feature = "large_board"))]
use ledger::Ledger;

#[cfg(feature = "threaded")]
extern crate crossbeam;
#[cfg(feature = "threaded")]
extern crate crossbeam_deque;
#[cfg(feature = "threaded")]
extern crate num_cpus;
#[cfg(feature = "threaded")]
pub mod solve_threaded;
#[cfg(feature = "threaded")]
pub use solve_threaded::solve;

#[cfg(not(feature = "threaded"))]
pub mod solve;
#[cfg(not(feature = "threaded"))]
pub use solve::solve;

#[cfg(not(feature = "threaded"))]
use std::collections::HashSet;

/// Data structure that contains the "stack" onto which words are built,
/// And the hash set of words that were found during a job.
#[cfg(not(feature = "threaded"))]
pub struct BoggleJob {
    x: isize,
    y: isize,
    buffer: String,
    solutions: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dict::dict;
    #[cfg(feature = "slow_board")]
    use trie::Node;

    #[test]
    fn sample_board() {
        let trie = dict("/usr/share/dict/words");
        let sample = sample_to_vecs(&[&['a', 'n'], &['t', 'd']]);
        let mut expected = result_to_vec(&["ant", "and", "tan", "tad"]);
        let board = Board::new(sample).unwrap();
        let mut result = solve(&board, &trie);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn standard_board() {
        let trie = dict("/usr/share/dict/words");
        let sample = sample_to_vecs(&[
            &['m', 'a', 'p', 'o'],
            &['e', 't', 'e', 'r'],
            &['d', 'e', 'n', 'i'],
            &['l', 'd', 'h', 'c'],
        ]);

        let board = Board::new(sample).unwrap();
        let mut expected = result_to_vec(&[
            "amt", "ape", "apt", "apter", "ate", "cheep", "cheer", "chi", "chin", "chirp", "deed",
            "deem", "deep", "deer", "den", "denier", "dent", "dented", "deter", "eat", "eaten",
            "eater", "edema", "eel", "end", "ended", "enrich", "enriched", "enter", "eta", "heed",
            "held", "hen", "hie", "hind", "hint", "hinted", "hire", "inc", "inch", "inched", "ind",
            "indeed", "inept", "int", "intel", "inter", "ire", "led", "lee", "leer", "lend",
            "lent", "let", "map", "mat", "mate", "mated", "meat", "med", "meddle", "meet", "met",
            "meta", "mete", "meted", "meteor", "meteoric", "meter", "neat", "need", "net", "niche",
            "open", "opened", "opt", "opted", "ore", "orient", "oriented", "pat", "pate", "pea",
            "peat", "pee", "peed", "peel", "pen", "pend", "pended", "pent", "per", "pet", "poet",
            "pore", "porn", "preen", "preteen", "pretend", "print", "printed", "pro", "ream",
            "reamed", "reap", "reed", "reel", "rein", "reined", "rend", "rent", "rented", "rep",
            "rich", "rind", "roe", "rope", "tam", "tame", "tamed", "tap", "tape", "taper", "tea",
            "team", "teamed", "tee", "teed", "teem", "teen", "teenier", "tel", "ten", "tend",
            "tended", "tern",
        ]);
        let mut result = solve(&board, &trie);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn q_board() {
        let trie = dict("/usr/share/dict/words");
        // The "expected" list here is accurate.  "queue" is indeed on the board: recall that 'q' is always
        // implicitly both "q" and "qu", so there are technically two "u"s on the board, it's just that
        // one of them can only ever be used as a letter following 'q', leaving the other one free to
        // be the latter 'u' in "queue" and "queues".
        let sample = sample_to_vecs(&[&['q', 'u', 'e'], &['e', 'e', 'y'], &['n', 's', 'r']]);
        let mut expected = result_to_vec(&[
            "eery", "eye", "eyes", "queen", "queens", "queer", "queers", "query", "ques", "queue",
            "queues", "rye", "see", "seen", "seer", "sneer", "yen", "yens", "yes", "yrs",
        ]);
        let board = Board::new(sample).unwrap();
        let mut result = solve(&board, &trie);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[cfg(feature = "slow_board")]
    #[test]
    fn slow_board() {
        let mut trie: Node<char> = Node::new();
        for i in 1..=16 {
            trie.insert(&mut std::iter::repeat('e').take(i))
        }
        let sample = sample_to_vecs(&[
            &['e', 'e', 'e', 'e'],
            &['e', 'e', 'e', 'e'],
            &['e', 'e', 'e', 'e'],
            &['e', 'e', 'e', 'e'],
        ]);

        let board = Board::new(sample).unwrap();
        let result = solve(&board, &trie);
        assert_eq!(result.len(), 14);
    }

    #[cfg(feature = "slow_board")]
    #[cfg(feature = "threaded")]
    #[test]
    fn threaded_slow_board() {
        let mut trie: Node<char> = Node::new();
        for i in 1..=16 {
            trie.insert(&mut std::iter::repeat('e').take(i))
        }
        let sample = sample_to_vecs(&[
            &['e', 'e', 'e', 'e'],
            &['e', 'e', 'e', 'e'],
            &['e', 'e', 'e', 'e'],
            &['e', 'e', 'e', 'e'],
        ]);

        let board = Board::new(sample).unwrap();
        let result = solve(&board, &trie);
        assert_eq!(result.len(), 14);
    }

    fn sample_to_vecs(arr: &[&[char]]) -> Vec<Vec<char>> {
        let mut res = Vec::new();
        for i in arr {
            let mut row = Vec::new();
            for j in *i {
                row.push(*j);
            }
            res.push(row);
        }
        res
    }

    fn result_to_vec(arr: &[&str]) -> Vec<String> {
        let mut res = Vec::new();
        for s in arr {
            res.push(s.to_string());
        }
        res
    }
}
