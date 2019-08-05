extern crate clap;

use boggle_generator::generate_boggle_board;
use clap::{App, Arg};
use std::str::FromStr;

/// Given a string and a separator, returns the two values
/// separated by the separator.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

pub fn main() {
    let matches = App::new("mkboggle")
        .version("0.1.0")
        .author("Elf M. Sternberg <elf.sternberg@gmail.com>")
        .about("Boggle™ board generator")
        .arg(
            Arg::with_name("size")
                .required(false)
                .long("size")
                .short("s")
                .default_value("4x4")
                .takes_value(true)
                .help("The size of the board to generate, example: 5x5")
        )
        .get_matches();

    let bounds =
        parse_pair(matches.value_of("size").unwrap(), 'x').expect("Error parsing board dimensions.");
    let board = generate_boggle_board(bounds.0, bounds.1);
    for row in board {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}
