When last we left our heroes, [they'd solved
Boggle!](https://elfsternberg.com/2019/08/09/solve-for-boggle/) and done
so with amazing speed.  The only thing left to do was go faster.

There are *two* strategies, actually, for going faster.  The first,
mind-bogglingly enough, is to *go the other way* with your search space:
building a graph of the board, for *each word in the dictionary* scan to
see if that word could be made from the board.

There is a size of board for which this strategy is actually faster than
the "search the board and check the word" strategy we're using, but I'll
tell you this: that board is bigger than 12x12.  *Even with* the
[two-phase
bitmap](https://elfsternberg.com/2019/08/08/data-structures-rust-lowly-bitmap/)
used for boards bigger than 8x8, a 12x12 board solves in only a second
or two.

I'm going to give much of the credit for this astounding speed to [The
Rust Programming Language](https://rust-lang.org) itself.  The allocator
for Rust is wicked clever about allocating memory resources while at the
same time preventing a whole host of errors common to C++.  Having
programmed mostly in Rust for the past nine months (with some Haskell
and Racket, and my day-to-day tool of Hy), I think I'd rather be work at
a coffee shop than go back to hacking C++ or Java.

Let's stick with our solution, but let's *multithread* it.  This article
will show you how I multithreaded the Boggle searcher using the
low-level Crossbeam toolkit and it's work queue extension, Deque.
Cossbeam is an amazing library that takes Rust's cleverness and applies
it to allocating CPU resources.

First thing: except for the `solve()` function, *nothing else in our
program has to change*.  Everything else is already thread-safe.  The
board and the dictionary are both read-only: every call to solve
launches the recursive solver solution for a *specific starting point*
on the board, with a new, unique Scanner object to store both the
successful matches (and prefix matches) and the bitmap to track the path
taken.

So the trick is to call `solveforpos()` in a thread.  However, the
average consumer laptop has four or eight cores, but the average Boggle™
board has more positions than four or eight.  We can't just launch as
many threads as we have positions to check.

It's also not efficient to say, "If we have four cores, let's just divvy
up the 4x4 board into 4 work groups and give each group to a different
thread."  This strategy is known as *job batching*, but it can lead to
inefficiencies.  One batch may be all small jobs, whereas another may be
all big jobs.  The small batches will be done and the cores idling while
waiting for the big jobs to finish.  We'll address that later.

Crossbeam provides a mechanism for dealing with this: *stealing*.  A
collection of queues is shared among the threads, and when one thread
finishes its local queue it can steal a job from another thread's
queue.  If that doesn't work, it can steal from a global queue.

For now, let's not worry about stealing from peers.  Let's just have a
global queue of `Job`s to tackle.  A Job encodes the starting position
on the Board for a given scan and the `Scanned` object from the previous
post that will collect the data.  The system is done when the queue of
jobs is empty.

## How to use Crossbeam-Deque

We're going to be using
[crossbeam](https://docs.rs/crossbeam/0.7.2/crossbeam/) and
[crossbeam-deque](https://docs.rs/crossbeam-deque/0.7.1/crossbeam_deque/),
`Injector` is the first object we're going to be using from
crossbeam. `Injector` is our deque (a first-in, first-out queue), and
we're loading our job descriptions into it.  It's a funny name: it
implies that it pushes jobs, but most of the time our worker threads
will actually be pulling jobs out of the `Injector`.

``` rust
pub fn solve_mt(board: &Board, threads: usize) -> Vec<String> {
    struct Job(isize, isize, Scanned);

    let work = &{
        let mut work: Injector<Job> = Injector::new();
        for x in 0..board.mx {
			for y in 0..board.my {
                work.push(Job(x, y,
                    Scanned::new("".to_string(), Ledger::new(board.mx, board.my)),
                ));
            }
        }
        work
    };
```
The outer `work` variable is a *reference* to the work queue, which
itself is now anonymous.  This is important because it lets Rust know
that `work` is going to be reaped at the end of this function while
still allowing multiple immutable references to `work` to be used.

Crossbeam has a nifty thread launching system with one powerful
abstraction I mentioned earlier: temporal scope.  The `scope()` function
allows for [structured
concurrency](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/),
guaranteeing that every thread launched within a scope has terminated
before the parent thread is allowed to proceed.  Inside a scope we're
free to `spawn()` as many threads as we like, but they will all end
before we leave the scope.

So: we need a place to record the results of any one search, and then we
need to launch the search, and at the end we need to collect all the
results and return them.  We also need to *find the job* we're going to
be processing, but I'm going to cheat and do that later.  Let's launch:
```rust 
    let mut solutions: Vec<String> = vec![];
    crossbeam::scope(|spawner| {
        let handles: Vec<ScopedJoinHandle<Vec<String>>> = (0..threads)
            .map(|_| {
                spawner.spawn(move |_| {
                    let mut solutions: Vec<String> = vec![];
                    let mut queue: Worker<Job> = Worker::new_fifo();
                    loop {
                        match find_task(&mut queue, &work) {
                            Some(mut job) => {
                                solveforpos(&board, (job.0, job.1), &mut job.2, &mut solutions);
                            }
                            None => break,
                        }
                    }
                    solutions
                })
            })
            .collect();
```
A couple of things stand out here.

First, the topmost `solutions` collection is outside the scope. We don't
want to lose the result of our work when the scope ends and Rust
reclaims the memory needed to do our processing.  The temporary data
generated during a search is ephemeral, but our final answers shouldn't
be.

Second, the `handles` collection is where our data actually ends up, and
we need to extract the results of a thread from it.  It's a fairly
complex type signature, but straightforward once you figure it out.

Third, we've got that `Worker` thing, which is also from crossbeam.
Crossbeam-deque has its 'job stealing' mechanism, in which all threads
in a scope can have multiple job queues and 'steal' work from each other
(usually half the items in a queue) as each thread finishes its own
queue.

We're not going to use this feature.  We're *just* going to have a
single scoped queue in `Injector` and pop jobs off.  But crossbeam wants
each thread to have a queue, so we're going to have it, but at run-time
that queue will have only one job in it, the *current* job.  When a
thread has completed one full search it will loop around and ask the
`Injector`, for another job.

Fourth, we have *another* `solutions` vector of strings; this is the one
that goes into the vector of `ScopedJoinHandle`s, and in the end we have
to merge and flatten them all.

And finally, a pro-tip: see that `.collect()` at the bottom?  I used
`Range.map()` to launch my threads, rather than a `for` loop.  And
`.map()` is lazy: it produces exactly *one* iteration, waits for it to
complete, and then goes on to produce the next.  Without `.collect()`,
the spawner produces one child thread which does all the work... and
then we're actually a little *slower* than the the single-threaded
version due to the overhead.  Using `.collect()` there forces `.map()`
to be eager, to try and collect all the threads at once and put their
results into `handles` immediately.

All right, we've run the threads and collected the results.  What's
next?  Extraction:
```
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
```
In the first line, when the parent scope hits the first `handle.join()`,
it waits until the first thread launched terminates, and then collects
its data.  If the second and third thread are also done, it immediately
collects those, otherwise it goes back into the waiting state until it's
merged the entire scope down.  The `.unwrap()` is there to handle error
conditions of the total concurrency handling process, but I'm going to
ignore it for now.

We then sort and de-duplicate our collection and return it.

Much of the point of this exercise is to simplify [the queue example
from crossbeam](https://docs.rs/crossbeam-deque/0.7.1/crossbeam_deque/).
The version of `find_task` on that page is esoteric and difficult to
read.  I wanted a simpler version that I could understand.  This is what
mine looks like:
```
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
```
A thread looking for work passes in both its local and the global
queues.  We attempt to pop the local job off, which should never exist;
it's just an empty container needed to pass this assertion.  We then
attempt to `steal` another job from the global queue.  If we're out of
jobs, we return `None`.  If we find a job, we return `Some(job)`.  And
finally, if the global queue was locked (due to another thread being in
the middle of getting work), we tell find_task to busy-wait and try
again.

That's it.  That's the whole of what that big mess on the
crossbeam-deque page is trying to tell you, only using `iter::repeat`
instead of `loop` and throwing in a whole bunch of stuff about stealing
jobs from other threads.  Which *you will need* someday if you write
lots of concurrent code.  For learning, however, you will not want it;
you want the simplest level of code you can hold in your head.  A loop
does that for me.

And that's it.  Your Boggle solver is now multi-threaded.  And the
results are impressive: With four threads, it's now about three times
faster, solving the board in about 350ns.  It's not four times faster;
there's allocation and processing overhead involved in separating out
the jobs.  But it's three times faster, which is a great improvement.











