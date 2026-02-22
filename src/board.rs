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

pub(crate) struct Scanned(String, Ledger);

impl Scanned {
    pub fn new(word: String, positions: Ledger) -> Scanned {
        Scanned(word, positions)
    }

    /// During the course of searching the board, the add() function receives
    /// a character and a position, and if the position has not been visited,
    /// creates a new aggregate with previous word + the new character.
    ///
    pub fn add(
        &mut self,
        c: char,
        (i, j): (isize, isize),
        skip_pos_check: bool,
    ) -> Option<Scanned> {
        if self.1.check(i, j) || skip_pos_check {
            return None;
        }

        let mut newword = self.0.to_string();
        newword.push(c);
        Some(Scanned::new(newword, self.1.mark(i, j)))
    }
}

/// A boggle game with a valid dictionary.
///
pub struct Board<'a> {
    board: Vec<Vec<char>>,
    /// The dictionary of words, in trie form
    pub words: &'a Node<char>,
    #[doc(hidden)]
    pub mx: isize,
    #[doc(hidden)]
    pub my: isize,
}

impl<'a> Board<'a> {
    /// Takes an n⨯m board of char, and a dictionary, and returns
    /// a new Board waiting to be solved.
    pub fn new(board: Vec<Vec<char>>, words: &Node<char>) -> Option<Board<'_>> {
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
        Some(Board {
            board,
            words,
            mx,
            my,
        })
    }
}

// Given:
// A board
// A position on the board
// A ledger of visited positions
// The current dictionary location
// The current collection of solutions

// 1. Find the position on the board.  If it has been visited, stop
// 2. Get the letter at that position.  Check the dictionary for "this is a valid letter from the current position in the dictionary."  If it is not, stop.
// 3. Append the letter to the current solution.  If the dictionary says this creates a word, record it.
// 4. For every valid position around the current one, attempt to recurse into it.

pub(crate) fn solveforpos<'a>(
    board: &Board<'a>,
    (x, y): (isize, isize),
    ledger: &mut Scanned,
    dictionary: &'a Node<char>,
    solutions: &mut HashSet<String>,
) {
    let c = board.board[x as usize][y as usize];
    if let Some(dictionary) = dictionary.0.get(&c) {
        innersolveforpos(c, board, (x, y), ledger, dictionary, solutions, false);
        if c == 'q' {
            if let Some(dictionary) = dictionary.0.get(&'u') {
                innersolveforpos('u', board, (x, y), ledger, dictionary, solutions, true);
            }
        }
    }
}

fn innersolveforpos<'a>(
    c: char,
    board: &Board<'a>,
    (x, y): (isize, isize),
    ledger: &mut Scanned,
    dictionary: &'a Node<char>,
    solutions: &mut HashSet<String>,
    skip_pos_check: bool,
) {
    match ledger.add(c, (x, y), skip_pos_check) {
        None => (),
        Some(mut ledger) => {
            if ledger.0.len() > 2 && dictionary.1 {
                solutions.insert(ledger.0.to_string());
            }

            if dictionary.0.is_empty() {
                return;
            }

            for (i, j) in POSITIONS.iter() {
                let (nx, ny) = (x + i, y + j);
                if nx >= 0 && nx < board.mx && ny >= 0 && ny < board.my {
                    solveforpos(board, (nx, ny), &mut ledger, dictionary, solutions);
                }
            }
        }
    }
}
