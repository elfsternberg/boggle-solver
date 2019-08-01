use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use boggle_solver::Board;
use boggle_solver::dict::dict;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        writeln!(std::io::stderr(), "Usage: boggle <path to board>").unwrap();
        std::process::exit(1);
    }

    let board = {
        let mut board: Vec<Vec<char>> = Vec::new();
        let f = File::open(&args[1]).expect("Unable to open board file.");
        let f = BufReader::new(f);
        for line in f.lines() {
            match line {
                Ok(line) => {
                    let v: Vec<char> = line.split(' ').map(|i| i.to_string().chars().next().unwrap()).collect();
                    board.push(v);
                }
                Err(_) => {
                    writeln!(std::io::stderr(), "Could not parse boggle board file").unwrap();
                    std::process::exit(1);
                }
            }
        }
        board
    };        
    
    let trie = dict();
    let mut board = Board::new(board, &trie).unwrap();
    let solutions = board.solve();
    for s in solutions {
        println!("{}", s);
    }
}

    
    
