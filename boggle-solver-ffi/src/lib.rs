extern crate boggle_solver;
extern crate libc;
extern crate regex;

use boggle_solver::dict::dict;
use regex::Regex;
use std::mem::transmute;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct Trie(boggle_solver::trie::Node<char>);

/// The expected input here is the *path* to the dictionary file.
#[no_mangle]
unsafe extern "C" fn dictionary_make(filepath: *const c_char) -> *const Trie {
    transmute(Box::new(Trie(dict(
        CStr::from_ptr(filepath).to_str().unwrap(),
    ))))
}

#[no_mangle]
unsafe extern "C" fn dictionary_destroy(trie: *const Trie) {
    let _drop_me: Box<Trie> = transmute(trie);
}

#[no_mangle]
unsafe extern "C" fn solve_for_dictionary(
    board_text: *const c_char,
    dictionary: *const Trie,
    found_words: *mut c_char,
) {
    let text: String = (CStr::from_ptr(board_text)).to_str().unwrap().to_owned();

    let board = {
        let re = Regex::new(r"[ \t]*").unwrap();
        let mut board: Vec<Vec<char>> = Vec::new();
        let mut found: Option<usize> = None;
        for line in text.lines() {
            let v: Vec<&str> = re.split(&line).collect();
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
        board
    };

    let solutions =
        boggle_solver::solve(&boggle_solver::Board::new(board, &(*dictionary).0).unwrap());
    let mut result = String::new();

    if !solutions.is_empty() {
        for solution in solutions {
            result.push_str(&solution);
            result.push('\n');
        }
    }

    let s = CString::new(result).unwrap();
    libc::strcpy(found_words, s.as_ptr());
}

#[no_mangle]
unsafe extern "C" fn solve(
    board_text: *const c_char,
    dictionary_path: *const c_char,
    found_words: *mut c_char,
) {
    let trie = dictionary_make(dictionary_path);
    solve_for_dictionary(board_text, trie, found_words);
    dictionary_destroy(trie);
}
