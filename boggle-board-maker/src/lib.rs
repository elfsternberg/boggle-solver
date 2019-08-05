extern crate rand;
use rand::Rng;

pub fn generate_boggle_board(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut dice = vec![
        vec!['S', 'R', 'E', 'L', 'A', 'C'],
        vec!['D', 'P', 'A', 'C', 'E', 'M'],
        vec!['Q', 'B', 'A', 'O', 'J', 'M'],
        vec!['D', 'U', 'T', 'O', 'K', 'N'],
        vec!['O', 'M', 'H', 'R', 'S', 'A'],
        vec!['E', 'I', 'F', 'E', 'H', 'Y'],
        vec!['B', 'R', 'I', 'F', 'O', 'X'],
        vec!['R', 'L', 'U', 'W', 'I', 'G'],
        vec!['N', 'S', 'O', 'W', 'E', 'D'],
        vec!['Y', 'L', 'I', 'B', 'A', 'T'],
        vec!['T', 'N', 'I', 'G', 'E', 'V'],
        vec!['T', 'A', 'C', 'I', 'T', 'O'],
        vec!['P', 'S', 'U', 'T', 'E', 'L'],
        vec!['E', 'P', 'I', 'S', 'H', 'N'],
        vec!['Y', 'K', 'U', 'L', 'E', 'G'],
        vec!['N', 'Z', 'E', 'V', 'A', 'D'],
    ];

    let mut rng = rand::thread_rng();
    let msize = rows * cols;
    while dice.len() < msize {
        let sample = (&dice[rng.gen_range(0, 16)]).to_vec();
        dice.push(sample);
    }

    let mut board: Vec<Vec<char>> = vec![];
    for _i in 0..rows {
        let mut row = vec![];
        for _j in 0..cols {
            let die = dice.remove(rng.gen_range(0, dice.len()));
            let ltr = die[rng.gen_range(0, 6)];
            row.push(ltr);
        }
        board.push(row);
    }
    board
}
