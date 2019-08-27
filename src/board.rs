#![deny(missing_docs)]

//! An aggregating structure for scanning a Boggle! board.

use crate::trie::Node;
use crate::Ledger;
use itertools::iproduct;


pub(in crate) struct Scanned(String, Ledger);

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
    words: &'a Node<char>,
    #[doc(hidden)]
    pub mx: isize,
    #[doc(hidden)]
    pub my: isize,
}

impl<'a> Board<'a> {
    /// Takes an n⨯m board of char, and a dictionary, and returns
    /// a new Board waiting to be solved.
    pub fn new(board: Vec<Vec<char>>, words: &Node<char>) -> Option<Board> {
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

pub(in crate) fn solveforpos(
    board: &Board,
    (x, y): (isize, isize),
    curr: &mut Scanned,
    solutions: &mut Vec<String>,
) {
    let c = board.board[x as usize][y as usize];
    innersolveforpos(c, board, (x, y), curr, solutions, false);
    if c == 'q' {
        innersolveforpos('u', board, (x, y), curr, solutions, true);
    }
}

fn innersolveforpos(
    c: char,
    board: &Board,
    (x, y): (isize, isize),
    curr: &mut Scanned,
    solutions: &mut Vec<String>,
    skip_pos_check: bool,
) {
    match curr.add(c, (x, y), skip_pos_check) {
        None => (),
        Some(mut newcurr) => {
            if newcurr.0.len() > 2 && board.words.find(&mut newcurr.0.chars()) {
                solutions.push(newcurr.0.to_string());
            }

            if !board.words.pref(&mut newcurr.0.chars()) {
                return;
            }

            iproduct!(-1..=1, -1..=1)
                .filter(|(i, j)| !((*i == 0) && (*j == 0)))
                .map(|(i, j)| (x + i, y + j))
                .filter(|(nx, ny)| *nx >= 0 && *nx < board.mx && *ny >= 0 && *ny < board.my)
                .for_each(|(nx, ny)| solveforpos(board, (nx, ny), &mut newcurr, solutions));
        }
    }
}
