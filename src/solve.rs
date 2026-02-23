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
use std::collections::HashSet;

/// Represents a boggle job. Each die on the board will be scanned sequentially,
/// but since they don't conflict we can scan the positions for as many threads
/// as we have available.
pub struct BoggleJob {
    x: isize,
    y: isize,
    buffer: String,
    solutions: HashSet<String>,
}

/// Solve the Boggle board
///
pub fn solve(board: &Board, dictionary: &Node<char>) -> Vec<String> {
    let mut work: Vec<BoggleJob> = iproduct!(0..(board.mx as u64), 0..(board.my as u64))
        .map(|(x, y)| BoggleJob {
            x: x as isize,
            y: y as isize,
            buffer: String::with_capacity(24),
            solutions: HashSet::new(),
        })
        .collect();

    for job in &mut work {
        // This is where the work queue goes.  Each job will be
        // independently run in a worker, and the results collated
        // together afterward.  This is the first step toward
        // map/reducing the solver.
        solveforpos(
            &board,
            &dictionary,
            (job.x, job.y),
            &mut job.buffer,
            Ledger::new(board.mx, board.my),
            &mut job.solutions,
        );
    }

    let mut solutions: HashSet<String> = HashSet::new();
    for job in &mut work {
        solutions.extend(job.solutions.iter().cloned())
    }

    let mut solutions: Vec<String> = solutions.into_iter().collect();
    solutions.sort();
    solutions
}
