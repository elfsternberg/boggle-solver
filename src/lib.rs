// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Boggle solver
///
/// The readme has more, but Boggle is a popular game released in 1972
/// in which a collection of 16 dice with letters printed on the sides
/// are tossed into a 4⨯4 grid and then the players have three minutes
/// to find as many valid words as they can (valid according to the
/// dictionary of choice (Americans typically use either Webster's or
/// the Scrabble North American dictionary).

pub mod dict;
mod trie;
use trie::Node;

#[cfg(not(feature="large_board"))]
struct Ledger(isize, isize, u64);

#[cfg(not(feature="large_board"))]
impl Ledger {
    pub fn new(x: isize, y:isize) -> Ledger { Ledger(x, y, 0) }

    #[inline]
    fn next(&self, ledger: u64) -> Ledger {
        Ledger(self.0, self.1, ledger)
    }

    #[inline]
    fn point(&self, x: isize, y: isize) -> u64 {
        1 << (self.1 * x + y)
    }
        
    pub fn mark(&self, x: isize, y: isize) -> Ledger {
        self.next(self.2 | self.point(x, y))
    }
    pub fn check(&self, x: isize, y:isize) -> bool {
        let v = self.point(x, y);
        self.2 & v == v
    }
}

#[cfg(feature="large_board")]
extern crate fsbitmap;

#[cfg(feature="large_board")]
use fsbitmap::FSBitmap;

#[cfg(feature="large_board")]
struct Ledger(isize, isize, FSBitmap);

#[cfg(feature="large_board")]
impl Ledger {
    pub fn new(x: isize, y:isize) -> Ledger { Ledger(x, y, FSBitmap::new((x * y) as usize)) }

    #[inline]
    fn next(&self, ledger: FSBitmap) -> Ledger {
        Ledger(self.0, self.1, ledger)
    }

    #[inline]
    fn point(&self, x: isize, y: isize) -> u64 {
        (self.0 * x + (y % self.1)) as u64
    }

    pub fn mark(&mut self, x: isize, y: isize) -> Ledger {
        let mut newmap = self.2.clone();
        newmap.mark(self.point(x, y) as usize);
        self.next(newmap)
    }
    pub fn check(&self, x: isize, y:isize) -> bool {
        self.2.check(self.point(x, y) as usize)
    }
}

/// An aggregating structure for scanning the board.
struct Scanned {
    positions: Ledger,
    word: String,
}

impl Scanned {
    pub fn new(word: String, positions: Ledger) -> Scanned {
        Scanned { word, positions }
    }

    /// During the course of searching the board, the add() function receives
    /// a character and a position, and if the position has not been visited,
    /// creates a new aggregate with previous word + the new character.
    pub fn add(&mut self, c: char, (i, j): (isize, isize), skip_pos_check: bool) -> Option<Scanned> {
        if self.positions.check(i, j) || skip_pos_check {
            return None;
        }

        let mut newword = self.word.to_string();
        newword.push(c);
        Some(Scanned::new(newword, self.positions.mark(i, j)))
    }
}

/// A boggle game with a valid dictionary.
pub struct Board<'a> {
    board: Vec<Vec<char>>,
    words: &'a Node<char>,
    mx: isize,
    my: isize,
}

impl<'a> Board<'a> {

    /// Takes an n⨯m board of char, and a dictionary, and returns
    /// a new Board waiting to be solved.
    ///
    pub fn new(board: Vec<Vec<char>>, words: &Node<char>) -> Option<Board> {
        if board.is_empty() {
            return None;
        }
        let my = board[1].len() as isize;
        let mx = board.len() as isize;
        Some(Board {
            board,
            words,
            mx,
            my,
        })
    }

}


/// Solve the Boggle board
///
pub fn solve(board: &Board) -> Vec<String> {

    fn solveforpos(board: &Board, (x, y): (isize, isize), curr: &mut Scanned, solutions: &mut Vec<String>) {
        let c = board.board[x as usize][y as usize];
        innersolveforpos(c, board, (x, y), curr, solutions, false);
        if c == 'q' {
            innersolveforpos('u', board, (x, y), curr, solutions, true);
        }
    }

    fn innersolveforpos(c: char, board: &Board, (x, y): (isize, isize), curr: &mut Scanned, solutions: &mut Vec<String>, skip_pos_check: bool) {
        match curr.add(c, (x, y), skip_pos_check) {
            None => return,
            Some(mut newcurr) => {
                if newcurr.word.len() > 2 && board.words.find(&mut newcurr.word.chars()) {
                    solutions.push(newcurr.word.to_string());
                }
                
                if !board.words.pref(&mut newcurr.word.chars()) {
                    return;
                }
                
                for i in -1..=1 {
                    for j in -1..=1 {
                        if !(i == 0 && j == 0) {  // Skip the current block!
                            let (nx, ny): (isize, isize) = (x as isize + i, y as isize + j);
                            if nx >= 0 && nx < board.mx && ny >= 0 && ny < board.my {
                                solveforpos(board, (nx, ny), &mut newcurr, solutions)
                            }
                        }
                    }
                }
            }
        }
    }

    let mut work = {
        let mut work: Vec::<(isize, isize, Scanned, Vec<String>)> = vec![];
        for x in 0..board.mx {
            for y in 0..board.my {
                work.push((x, y, Scanned::new("".to_string(), Ledger::new(board.mx, board.my)), vec![]));
            }
        }
        work
    };

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
        let sample = sample_to_vecs(
            &[&['q','u','e'],
              &['e','e','y'],
              &['n','s','r']]);
        let mut expected = result_to_vec(
            &["eery", "eye", "eyes", "queen", "queens", "queer", "queers", "query",
              "rye", "see", "seen", "seer", "sneer", "yen", "yens", "yes"]);
        let board = Board::new(sample, &trie).unwrap();
        let mut result = solve(&board);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
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
