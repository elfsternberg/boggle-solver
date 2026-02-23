#![deny(missing_docs)]

//! An aggregating structure for scanning a Boggle! board.

use std::collections::HashSet;

use crate::Ledger;
use crate::trie::Node;
use itertools::iproduct;
use lazy_static::lazy_static;

lazy_static! {
static ref POSITIONS: Vec<(isize, isize)> = iproduct!(-1..=1, -1..=1)
    // Skip the center position; we just want neighbors
        .filter(|(i, j)| !((*i == 0) && (*j == 0)))
        .collect();
}

/// A boggle board: size and content
///
pub struct Board {
    board: Vec<Vec<char>>,
    #[doc(hidden)]
    pub mx: isize,
    #[doc(hidden)]
    pub my: isize,
}

impl Board {
    /// Takes an n⨯m board of char, and a dictionary, and returns
    /// a new Board waiting to be solved.
    pub fn new(board: Vec<Vec<char>>) -> Option<Board> {
        if board.is_empty() {
            return None;
        }
        if board[0].is_empty() {
            return None;
        }

        let my = board[0].len();
        if board.iter().any(|b| b.len() != my) {
            return None;
        }

        let my = my as isize;
        let mx = board.len() as isize;
        Some(Board { board, mx, my })
    }
}

// This two-step dance is needed to check for a single special case: the `qu` face on one of the
// boggle die.

pub(crate) fn solveforpos(
    board: &Board,
    node: &Node<char>,
    (x, y): (isize, isize),
    buffer: &mut String,
    ledger: Ledger,
    solutions: &mut HashSet<String>,
) {
    let c = board.board[x as usize][y as usize];
    // Get the current position and solve for it. If it's a 'q', treat it as being both 'q' and 'qu'
    // and solve for that.  This handles words like 'burqa' and 'tranq'.
    if let Some(node) = node.next(&c) {
        buffer.push(c);
        innersolveforpos(board, node, (x, y), buffer, ledger, solutions, false);
        if c == 'q' {
            if let Some(node) = node.next(&'u') {
                buffer.push('u');
                innersolveforpos(board, node, (x, y), buffer, ledger, solutions, false);
                buffer.pop();
            }
        }
        buffer.pop();
    }
}

fn innersolveforpos(
    board: &Board,
    node: &Node<char>,
    (x, y): (isize, isize),
    buffer: &mut String,
    ledger: Ledger,
    solutions: &mut HashSet<String>,
    skip_pos_check: bool,
) {
    // If we've already visited this position, we can't use the letter there.
    if !skip_pos_check && ledger.check(x, y) {
        return;
    }

    // Push the letter into the current solution buffer
    // If the dictionary says this node represents a word, push a copy of the solution buffer
    // into the solution set.
    if buffer.len() > 2 && node.is_word() {
        solutions.insert(buffer.to_string());
    }
    // If the dictionary says there are no possible words that can be formed beyond this
    // sequence, exit this scan.
    if node.is_leaf() {
        return;
    }

    // sequentially solve for next positions.
    let ledger = ledger.mark(x, y);
    for (i, j) in POSITIONS.iter() {
        let (nx, ny) = (x + i, y + j);
        if nx >= 0 && nx < board.mx && ny >= 0 && ny < board.my {
            solveforpos(board, node, (nx, ny), buffer, ledger, solutions);
        }
    }
}
