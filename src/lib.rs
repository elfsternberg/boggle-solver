pub mod dict;
mod trie;
use trie::Node;

struct Scanned {
    positions: Vec<(isize, isize)>,
    word: String,
}

impl Scanned {
    pub fn new(word: String, positions: Vec<(isize, isize)>) -> Scanned {
        Scanned { word, positions }
    }

    pub fn add(&mut self, c: char, (i, j): (isize, isize)) -> Option<Scanned> {
        match self.positions.contains(&(i, j)) {
            true => None,
            false => {
                let mut newpos = self.positions.to_vec();
                newpos.push((i, j));
                let mut newword = self.word.to_string();
                newword.push(c);
                Some(Scanned::new(newword, newpos))
            }
        }
    }
}

pub struct Board<'a> {
    board: Vec<Vec<char>>,
    words: &'a Node,
    mx: isize,
    my: isize,
    solutions: Vec<String>,
}

impl<'a> Board<'a> {
    pub fn new(board: Vec<Vec<char>>, words: &Node) -> Option<Board> {
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
            solutions: Vec::new(),
        })
    }

    fn solveforpos(&mut self, posx: isize, posy: isize, curr: &mut Scanned) {
        match curr.add(self.board[posx as usize][posy as usize], (posx, posy)) {
            None => return,
            Some(mut curr) => {
                if curr.word.len() > 2 && self.words.find(&mut curr.word.chars()) {
                    self.solutions.push(curr.word.to_string());
                }

                if !self.words.pref(&mut curr.word.chars()) {
                    return;
                }

                for x in -1..=1 {
                    for y in -1..=1 {
                        if !(y == 0 && x == 0) {
                            let (nx, ny): (isize, isize) = (posx as isize + x, posy as isize + y);
                            if nx >= 0 && nx < self.mx && ny >= 0 && ny < self.my {
                                self.solveforpos(nx, ny, &mut curr)
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn solve(&mut self) -> Vec<String> {
        for x in 0..self.mx {
            for y in 0..self.my {
                let mut possibles = Scanned::new("".to_string(), Vec::new());
                self.solveforpos(x, y, &mut possibles)
            }
        }
        self.solutions.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dict::dict;

    #[test]
    fn sample_board() {
        let trie = dict();
        let sample = sample_to_vecs(&[&['a', 'n'], &['t', 'd']]);
        let mut expected = result_to_vec(&["ant", "and", "tan", "tad"]);
        let mut board = Board::new(sample, &trie).unwrap();
        let mut result = board.solve();
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn standard_board() {
        let trie = dict();
        let sample = sample_to_vecs(&[
            &['m', 'a', 'p', 'o'],
            &['e', 't', 'e', 'r'],
            &['d', 'e', 'n', 'i'],
            &['l', 'd', 'h', 'c'],
        ]);

        let mut board = Board::new(sample, &trie).unwrap();
        let mut expected = result_to_vec(&[
            "map", "mat", "mate", "mated", "mate", "mate", "mated", "mated", "meat", "met", "mete",
            "meteor", "meteoric", "meter", "mete", "meted", "meted", "meddle", "meet", "apt",
            "apter", "ape", "ate", "ate", "ate", "pat", "pate", "pate", "pate", "poet", "pore",
            "porn", "pea", "peat", "pet", "per", "pee", "peed", "peel", "peed", "pen", "pent",
            "pended", "pro", "preteen", "pretend", "preen", "print", "printed", "printed",
            "printed", "opt", "opted", "opted", "opted", "open", "opened", "opened", "ore",
            "orient", "oriented", "oriented", "oriented", "eat", "eater", "eaten", "eaten", "eta",
            "eel", "tam", "tame", "tamed", "tap", "tape", "taper", "tea", "team", "tee", "teed",
            "teen", "teenier", "teed", "tea", "team", "teamed", "tern", "tee", "teed", "teen",
            "teed", "ten", "tend", "tended", "tee", "teem", "teed", "tee", "teen", "ten", "tend",
            "eat", "eaten", "eta", "eel", "enrich", "enriched", "enriched", "end", "ended", "rope",
            "roe", "ream", "reamed", "reap", "rep", "reed", "reel", "reed", "rent", "rented",
            "rented", "rented", "rend", "rein", "reined", "reined", "rind", "rich", "deter",
            "deed", "deem", "deter", "deep", "deer", "den", "dent", "denier", "eta", "edema",
            "enter", "enrich", "end", "neat", "net", "need", "need", "need", "net", "niche", "ire",
            "inter", "inept", "indeed", "inch", "inched", "inched", "lee", "let", "lee", "leer",
            "led", "lent", "lend", "led", "deem", "deed", "deter", "deep", "deer", "den", "dent",
            "dented", "denier", "heed", "hen", "held", "held", "hie", "hire", "hint", "hinted",
            "hinted", "hinted", "hind", "cheep", "cheer", "chi", "chirp", "chin",
        ]);
        let mut result = board.solve();
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
