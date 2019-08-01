extern crate clap;
extern crate regex;

use boggle_solver::dict::dict;
use boggle_solver::Board;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use clap::{Arg, App};
use regex::Regex;

pub fn main() {
 let matches = App::new("boggle")
        .version("0.1.0")
        .author("Elf M. Sternberg <elf.sternberg@gmail.com>")
        .about("Boggle™ solver written in Rust")
        .arg(Arg::with_name("BOARD")
             .help("The Boggle board to analyze")
             .required(true)
             .index(1))
        .arg(Arg::with_name("dict")
             .required(false)
             .long("dict")
             .short("d")
             .default_value("/usr/share/dict/words")
             .takes_value(true)
             .help("Dictionary to use"))
        .get_matches();

    let dpath = matches.value_of("dict").unwrap();
    let bpath = matches.value_of("BOARD").unwrap();
    let trie = dict(dpath);
    
    let board = {
        let re = Regex::new(r"[ \t]*").unwrap();
        let mut board: Vec<Vec<char>> = Vec::new();
        let f = File::open(bpath).expect("Unable to open board file.");
        let f = BufReader::new(f);
        let mut found: Option<usize> = None;
        for line in f.lines() {
            match line {
                Ok(line) => {
                    let v: Vec<&str> = re.split(&line).collect();
                    let v: Vec<char> = v.into_iter()
                                        .map(|i| i.to_string())
                                        .filter(|i| !i.is_empty())
                                        .map(|i| i.chars().next().unwrap())
                                        .collect();
                    match found {
                        Some(c) => if !v.is_empty() && c != v.len() {
                            writeln!(std::io::stderr(), "Boggle board rows must all be the same length").unwrap();
                            std::process::exit(1);
                        },
                        None => {
                            found = Some(v.len());
                        }
                    };
                    if !v.is_empty() {
                        board.push(v);
                    }
                }
                Err(_) => {
                    writeln!(std::io::stderr(), "Could not parse boggle board file").unwrap();
                    std::process::exit(1);
                }
            }
        }
        board
    };

    let mut board = Board::new(board, &trie).unwrap();
    let solutions = board.solve();
    for s in solutions {
        println!("{}", s);
    }
}
