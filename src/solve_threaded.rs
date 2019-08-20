use crate::board::{solveforpos, Board, Scanned};
use crate::Ledger;
use crossbeam::thread::ScopedJoinHandle;
use crossbeam_deque::{Injector, Steal, Worker};
use ndranges::ndrange;
use num_cpus;

fn find_task<T>(local: &mut Worker<T>, global: &Injector<T>) -> Option<T> {
    match local.pop() {
        Some(job) => Some(job),
        None => loop {
            match global.steal() {
                Steal::Success(job) => break Some(job),
                Steal::Empty => break None,
                Steal::Retry => {}
            }
        },
    }
}

/// Solve the Boggle board
///
/// Runs the solve algorithm, using as many CPUs as requested.
///
pub fn solve_mt(board: &Board, threads: usize) -> Vec<String> {
    struct Job(isize, isize, Scanned);

    let work = &{
        let work: Injector<Job> = Injector::new();
        ndrange(0..(board.mx as u64), 0..(board.my as u64))
            .into_iter()
            .for_each(|(x, y)| {
                work.push(Job(
                    x as isize,
                    y as isize,
                    Scanned::new("".to_string(), Ledger::new(board.mx, board.my)),
                ))
            });
        work
    };

    // Having to predefine the solutions object outside the scope so
    // that it's available during the join is a little awkward and
    // un-functional.
    //
    // Also, the collect() down there at the bottom of the map() function is
    // absolutely necessary; without it, the spawner only spawns once and
    // map waits for that to finish before spawning the other threads.  By
    // using collect(), we force map out of laziness and into eagerness, and
    // generate the threads before starting the work.
    let mut solutions: Vec<String> = vec![];
    crossbeam::scope(|spawner| {
        let handles: Vec<ScopedJoinHandle<Vec<String>>> = (0..threads)
            .map(|_| {
                spawner.spawn(move |_| {
                    let mut solutions: Vec<String> = vec![];
                    let mut queue: Worker<Job> = Worker::new_fifo();
                    while let Some(mut job) = find_task(&mut queue, &work) {
                        solveforpos(&board, (job.0, job.1), &mut job.2, &mut solutions);
                    }
                    solutions
                })
            })
            .collect();

        solutions = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .flatten()
            .collect()
    })
    .unwrap();

    solutions.sort();
    solutions.dedup();
    solutions
}

/// Solve the Boggle board
///
/// Runs the solve algorithm, using half the CPUs avaialable on the
/// current system.  For more fine-grained control, use `solve_mt`
/// instead.
///
pub fn solve(board: &Board) -> Vec<String> {
    let threads = num_cpus::get();
    solve_mt(board, threads / 2)
}
