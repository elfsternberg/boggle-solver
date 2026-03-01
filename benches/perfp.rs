#[macro_use]
extern crate criterion;

use criterion::Criterion;

use boggle_solver::dict::from_list;
use boggle_solver::{Board, solve};

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

fn eight_threads_standard_board(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let trie = from_list(&[
        "eee",
        "eeee",
        "eeeee",
        "eeeeee",
        "eeeeeee",
        "eeeeeeee",
        "eeeeeeeee",
        "eeeeeeeeee",
        "eeeeeeeeeee",
        "eeeeeeeeeeee",
        "eeeeeeeeeeeee",
        "eeeeeeeeeeeeee",
        "eeeeeeeeeeeeeee",
        "eeeeeeeeeeeeeeee",
        "eeeeeeeeeeeeeeeee",
    ]);

    c.bench_function(
        "eight threads, pathological board and dictionary",
        move |b| {
            b.iter(|| {
                let sample = sample_to_vecs(&[
                    &['e', 'e', 'e', 'e'],
                    &['e', 'e', 'e', 'e'],
                    &['e', 'e', 'e', 'e'],
                    &['e', 'e', 'e', 'e'],
                ]);
                let board = Board::new(sample).unwrap();
                let _result = solve(&board, &trie);
            })
        },
    );
}

// fn two_threads_standard_board(c: &mut Criterion) {
//     rayon::ThreadPoolBuilder::new()
//         .num_threads(2)
//         .build_global()
//         .unwrap();
//     let trie = dict("/usr/share/dict/words");
//     c.bench_function("Two_Threads, standard_board", move |b| {
//         b.iter(|| {
//             let sample = sample_to_vecs(&[
//                 &['m', 'a', 'p', 'o'],
//                 &['e', 't', 'e', 'r'],
//                 &['d', 'e', 'n', 'i'],
//                 &['l', 'd', 'h', 'c'],
//             ]);
//             let board = Board::new(sample).unwrap();
//             let _result = solve(&board, &trie);
//         })
//     });
// }
//
//fn four_threads_standard_board(c: &mut Criterion) {
//    rayon::ThreadPoolBuilder::new()
//        .num_threads(4)
//        .build_global()
//        .unwrap();
//    let trie = dict("/usr/share/dict/words");
//    c.bench_function("Four_threads, standard_board", move |b| {
//        b.iter(|| {
//            let sample = sample_to_vecs(&[
//                &['m', 'a', 'p', 'o'],
//                &['e', 't', 'e', 'r'],
//                &['d', 'e', 'n', 'i'],
//                &['l', 'd', 'h', 'c'],
//            ]);
//            let board = Board::new(sample).unwrap();
//            let _result = solve(&board, &trie);
//        })
//    });
//}
//
//fn eight_threads_standard_board(c: &mut Criterion) {
//    rayon::ThreadPoolBuilder::new()
//        .num_threads(8)
//        .build_global()
//        .unwrap();
//    let trie = dict("/usr/share/dict/words");
//    c.bench_function("Eight_threads, standard_board", move |b| {
//        b.iter(|| {
//            let sample = sample_to_vecs(&[
//                &['m', 'a', 'p', 'o'],
//                &['e', 't', 'e', 'r'],
//                &['d', 'e', 'n', 'i'],
//                &['l', 'd', 'h', 'c'],
//            ]);
//            let board = Board::new(sample).unwrap();
//            let _result = solve(&board, &trie);
//        })
//    });
//}
//
criterion_group!(benches, eight_threads_standard_board);
criterion_main!(benches);
