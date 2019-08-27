extern crate clap;
extern crate regex;

use boggle_solver::dict::dict;
use boggle_solver::{solve, Board};

use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};
use regex::Regex;

fn main() {
    let matches = App::new("solveboggle")
        .version("0.1.0")
        .author("Elf M. Sternberg <elf.sternberg@gmail.com>")
        .about("Boggle™ solver written in Rust")
        .arg(
            Arg::with_name("BOARD")
                .help("The Boggle board to analyze")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("dict")
                .required(false)
                .long("dict")
                .short("d")
                .default_value("/usr/share/dict/words")
                .takes_value(true)
                .help("Dictionary to use"),
        )
        .arg(
            Arg::with_name("score")
                .required(false)
                .long("score")
                .short("s")
                .help("Tally up total score"),
        )
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
                    // The map(...unwrap())s are safe here because we've
                    // already made the determination that these
                    // things exist in the prior filter pass.
                    let v: Vec<char> = v
                        .into_iter()
                        .map(|i| i.to_string())
                        .filter(|i| !i.is_empty())
                        .map(|i| i.chars().next().unwrap())
                        .filter(|i| i.is_ascii() && i.is_alphabetic())
                        .map(|i| i.to_lowercase().next().unwrap())
                        .collect();
                    match found {
                        Some(c) => {
                            if !v.is_empty() && c != v.len() {
                                eprintln!("Boggle board rows must all be the same length");
                                std::process::exit(1);
                            }
                        }
                        None => {
                            found = Some(v.len());
                        }
                    };
                    if !v.is_empty() {
                        board.push(v);
                    }
                }
                Err(_) => {
                    eprintln!("Could not parse boggle board file");
                    std::process::exit(1);
                }
            }
        }
        board
    };

    let board = Board::new(board, &trie).unwrap();
    let solutions = solve(&board);

    if matches.is_present("score") {
        let mut tally = 0;
        for s in solutions {
            let score = match s.len() {
                3 => 1,
                4 => 1,
                5 => 2,
                6 => 3,
                7 => 5,
                _ => 11,
            };
            println!(
                "{word:>width$}: {score}",
                word = s,
                width = 17,
                score = score
            );
            tally += score
        }
        println!("\nTotal: {}", tally);
    } else {
        for s in solutions {
            println!("{}", s);
        }
    }
}
