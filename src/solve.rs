#![deny(missing_docs)]

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Boggle solver: Main function
//!
//! Given a board and a dictionary, create a work queue and
//! solve for each position documented in the queue.

use crate::board::{solveforpos, Board, Scanned};
use crate::Ledger;
use itertools::iproduct;

/// Solve the Boggle board
///
pub fn solve(board: &Board) -> Vec<String> {
    let mut work: Vec<(isize, isize, Scanned, Vec<String>)> =
        iproduct!(0..(board.mx as u64), 0..(board.my as u64))
            .map(|(x, y)| {
                (
                    x as isize,
                    y as isize,
                    Scanned::new("".to_string(), Ledger::new(board.mx, board.my)),
                    vec![],
                )
            })
            .collect();

    for job in &mut work {
        // This is where the work queue goes.  Each job will be
        // independently run in a worker, and the results collated
        // together afterward.  This is the first step toward
        // map/reducing the solver.
        solveforpos(&board, (job.0, job.1), &mut job.2, &mut job.3);
    }

    let mut solutions: Vec<String> = vec![];
    for job in &mut work {
        solutions.extend(job.3.iter().cloned())
    }

    solutions.sort();
    solutions.dedup();
    solutions
}
