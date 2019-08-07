#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use boggle_solver::dict::dict;
use boggle_solver::{solve, solve_mt, Board};

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

fn single_thread_standard_board(c: &mut Criterion) {
    let trie = dict("/usr/share/dict/words");
    c.bench_function("Single_threaded, standard_board", move |b| {
        b.iter(|| {
            let sample = sample_to_vecs(&[
                &['m', 'a', 'p', 'o'],
                &['e', 't', 'e', 'r'],
                &['d', 'e', 'n', 'i'],
                &['l', 'd', 'h', 'c'],
            ]);
            let board = Board::new(sample, &trie).unwrap();
            let _result = solve(&board);
        })
    });
}

fn two_threads_standard_board(c: &mut Criterion) {
    let trie = dict("/usr/share/dict/words");
    c.bench_function("Two_Threads, standard_board", move |b| {
        b.iter(|| {
            let sample = sample_to_vecs(&[
                &['m', 'a', 'p', 'o'],
                &['e', 't', 'e', 'r'],
                &['d', 'e', 'n', 'i'],
                &['l', 'd', 'h', 'c'],
            ]);
            let board = Board::new(sample, &trie).unwrap();
            let _result = solve_mt(&board, 2);
        })
    });
}

fn four_threads_standard_board(c: &mut Criterion) {
    let trie = dict("/usr/share/dict/words");
    c.bench_function("Four_threads, standard_board", move |b| {
        b.iter(|| {
            let sample = sample_to_vecs(&[
                &['m', 'a', 'p', 'o'],
                &['e', 't', 'e', 'r'],
                &['d', 'e', 'n', 'i'],
                &['l', 'd', 'h', 'c'],
            ]);
            let board = Board::new(sample, &trie).unwrap();
            let _result = solve_mt(&board, 4);
        })
    });
}

fn eight_threads_standard_board(c: &mut Criterion) {
    let trie = dict("/usr/share/dict/words");
    c.bench_function("Eight_threads, standard_board", move |b| {
        b.iter(|| {
            let sample = sample_to_vecs(&[
                &['m', 'a', 'p', 'o'],
                &['e', 't', 'e', 'r'],
                &['d', 'e', 'n', 'i'],
                &['l', 'd', 'h', 'c'],
            ]);
            let board = Board::new(sample, &trie).unwrap();
            let _result = solve_mt(&board, 8);
        })
    });
}

criterion_group!(
    benches,
    single_thread_standard_board,
    two_threads_standard_board,
    four_threads_standard_board,
    eight_threads_standard_board
);
criterion_main!(benches);
