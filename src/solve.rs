use crate::board::{solveforpos, Board, Scanned};
use crate::Ledger;

/// Solve the Boggle board
///
pub fn solve(board: &Board) -> Vec<String> {
    let mut work = {
        let mut work: Vec<(isize, isize, Scanned, Vec<String>)> = vec![];
        for x in 0..board.mx {
            for y in 0..board.my {
                work.push((
                    x,
                    y,
                    Scanned::new("".to_string(), Ledger::new(board.mx, board.my)),
                    vec![],
                ));
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
