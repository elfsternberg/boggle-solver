// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod dict;
/// Boggle solver
///
/// The readme has more, but Boggle is a popular game released in 1972
/// in which a collection of 16 dice with letters printed on the sides
/// are tossed into a 4⨯4 grid and then the players have three minutes
/// to find as many valid words as they can (valid according to the
/// dictionary of choice (Americans typically use either Webster's or
/// the Scrabble North American dictionary).
mod trie;

pub mod board;
pub use board::Board;

#[cfg(feature = "large_board")]
extern crate fsbitmap;
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
#[cfg(feature = "threaded")]
pub use solve_threaded::solve_mt;

#[cfg(not(feature = "threaded"))]
pub mod solve;
#[cfg(not(feature = "threaded"))]
pub use solve::solve;

#[cfg(test)]
mod tests {
    use super::*;
    use dict::dict;

    #[test]
    fn sample_board() {
        let trie = dict("/usr/share/dict/words");
        let sample = sample_to_vecs(&[&['a', 'n'], &['t', 'd']]);
        let mut expected = result_to_vec(&["ant", "and", "tan", "tad"]);
        let board = Board::new(sample, &trie).unwrap();
        let mut result = solve(&board);
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

        let board = Board::new(sample, &trie).unwrap();
        let mut expected = result_to_vec(&[
            "ape", "apt", "apter", "ate", "cheep", "cheer", "chi", "chin", "chirp", "deed", "deem",
            "deep", "deer", "den", "denier", "dent", "dented", "deter", "eat", "eaten", "eater",
            "edema", "eel", "end", "ended", "enrich", "enriched", "enter", "eta", "heed", "held",
            "hen", "hie", "hind", "hint", "hinted", "hire", "inch", "inched", "indeed", "inept",
            "inter", "ire", "led", "lee", "leer", "lend", "lent", "let", "map", "mat", "mate",
            "mated", "meat", "meddle", "meet", "met", "mete", "meted", "meteor", "meteoric",
            "meter", "neat", "need", "net", "niche", "open", "opened", "opt", "opted", "ore",
            "orient", "oriented", "pat", "pate", "pea", "peat", "pee", "peed", "peel", "pen",
            "pended", "pent", "per", "pet", "poet", "pore", "porn", "preen", "preteen", "pretend",
            "print", "printed", "pro", "ream", "reamed", "reap", "reed", "reel", "rein", "reined",
            "rend", "rent", "rented", "rep", "rich", "rind", "roe", "rope", "tam", "tame", "tamed",
            "tap", "tape", "taper", "tea", "team", "teamed", "tee", "teed", "teem", "teen",
            "teenier", "ten", "tend", "tended", "tern",
        ]);
        let mut result = solve(&board);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn q_board() {
        let trie = dict("/usr/share/dict/words");
        let sample = sample_to_vecs(&[&['q', 'u', 'e'], &['e', 'e', 'y'], &['n', 's', 'r']]);
        let mut expected = result_to_vec(&[
            "eery", "eye", "eyes", "queen", "queens", "queer", "queers", "query", "rye", "see",
            "seen", "seer", "sneer", "yen", "yens", "yes",
        ]);
        let board = Board::new(sample, &trie).unwrap();
        let mut result = solve(&board);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    // This test is no different from the q_board() test above;
    // I just wanted a visual affirmation that this test showed
    // up only when the threaded feature was enabled, in order
    // to show that the feature was being used as expected.
    #[cfg(feature = "threaded")]
    #[test]
    fn threaded_board() {
        let trie = dict("/usr/share/dict/words");
        let sample = sample_to_vecs(&[&['q', 'u', 'e'], &['e', 'e', 'y'], &['n', 's', 'r']]);
        let mut expected = result_to_vec(&[
            "eery", "eye", "eyes", "queen", "queens", "queer", "queers", "query", "rye", "see",
            "seen", "seer", "sneer", "yen", "yens", "yes",
        ]);
        let board = Board::new(sample, &trie).unwrap();
        let mut result = solve_mt(&board, 2);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[cfg(feature = "large_board")]
    #[test]
    fn large_board() {
        let trie = dict("/usr/share/dict/words");
        let sample = sample_to_vecs(&[
            &['d', 'y', 's', 'o', 'i', 'm', 'v', 'n', 'o', 'y', 'c', 'o'],
            &['b', 'i', 'b', 'e', 'm', 'k', 'q', 'd', 'e', 'i', 'f', 'e'],
            &['e', 'k', 'l', 'p', 's', 'd', 'l', 'e', 'p', 'g', 'n', 'o'],
            &['i', 'd', 'e', 'n', 'b', 'a', 'r', 'a', 'k', 'i', 'e', 't'],
            &['s', 'p', 't', 'r', 'n', 'o', 'a', 'y', 'e', 'a', 'a', 'l'],
            &['n', 'n', 'w', 'b', 'y', 't', 'o', 'o', 'w', 'n', 'o', 'u'],
            &['h', 's', 'b', 'd', 'l', 'p', 'q', 'b', 'q', 'u', 'o', 'u'],
            &['c', 'r', 't', 'a', 'g', 'b', 'l', 'h', 'h', 'a', 'z', 's'],
            &['d', 'e', 'c', 'w', 'g', 'e', 'a', 'm', 'w', 'd', 'b', 'a'],
            &['r', 'q', 'm', 's', 'e', 'u', 'e', 'g', 'n', 'o', 't', 'i'],
            &['i', 'e', 's', 'a', 'a', 'n', 'p', 'v', 'e', 's', 'p', 'u'],
            &['n', 'p', 't', 'e', 'y', 'd', 'x', 'w', 'l', 'g', 'k', 'c'],
        ]);
        let board = Board::new(sample, &trie).unwrap();
        let mut result = solve(&board);
        result.sort();
        // I'm not going to try and keep the entire list here; just the count
        // is sufficient to ensure the ledger isn't broken for large sets.
        assert_eq!(result.len(), 980);
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
