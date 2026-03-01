#![deny(missing_docs)]
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Boggle solver: Main function
//!
//! Given a board and a dictionary, create a work queue and
//! solve for each position documented in the queue.

use crate::Ledger;
use crate::board::{Board, solveforpos};
use crate::trie::Node;
use itertools::iproduct;
use rayon::prelude::*;
use std::collections::HashSet;

/// Solve the Boggle board
///
/// Runs the solve algorithm, using as many CPUs as requested.
///
pub fn solve(board: &Board, dictionary: &Node<char>) -> Vec<String> {
    let positions: Vec<(u64, u64)> =
        iproduct!(0..(board.mx as u64), 0..(board.my as u64)).collect();

    let results: Vec<HashSet<String>> = positions
        .into_par_iter()
        .map(|(x, y)| {
            let mut solutions: HashSet<String> = HashSet::new();
            solveforpos(
                &board,
                &dictionary,
                (x as isize, y as isize),
                &mut String::with_capacity(24),
                Ledger::new(board.mx, board.my),
                &mut solutions,
            );
            solutions
        })
        .collect();

    let mut solutions: HashSet<String> = HashSet::new();
    for solution in results.into_iter() {
        solutions.extend(solution.iter().cloned());
    }

    let mut solutions: Vec<String> = solutions.into_iter().collect();
    solutions.sort();
    solutions
}
