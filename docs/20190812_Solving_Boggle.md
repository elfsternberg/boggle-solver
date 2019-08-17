All right, so now I've got a
[bitmap](https://elfsternberg.com/2019/08/08/data-structures-rust-lowly-bitmap/)
for tracking the path I've taken on the board, and I've got a
[trie](https://elfsternberg.com/2019/08/05/data-structures-tries-in-rust/)
that can determine whether or not a sequence of letters is a word (or
the prefix of a word) from a specified dictionary.

Let's solve Boggle™.

A Boggle board is a 4x4 grid where each cell is filled with one of
sixteen dice.  Each die has a letter.  The typical strategy is to start
at a letter and scan its neighbors for patterns that might register as
"a word."  We'll use the same technique.

<ol>
<li>Start with a letter</li>
<li>Scan one neighbor, creating a two-letter word</li>
<li>If that word is a whole word, add it to the solutions list</li>
<li>If that word is a prefix of a word in the dictionary
<ol>
<li> Pick a neighbor that we have not visited this scan</li>
<li> If there is one, make a three-letter word, and recurse to 3.</li>
</ol></li>
<li>Terminate when the "word" is not a prefix of any word or there are
no valid neighbors to visit</li>
</ol>

So we need a structure to contain the word we've scanned, and the
Ledger.  Let's call it `Scanned`.  The only special thing we need for
`Scanned` is that for every step, it needs to *clone* the Ledger; that
is, we get a new copy of `Scanned` for each neighboring letter, and
recurse down *that* path with *that* assembled word.  When the search
terminates (due to exhaustion of the search space or the failure of the
prefix), we will then use the copy of Scanned at this point in the stack
to create a new search with the *next* neighbor, and so forth.  This is
a standard recursive strategy.

The result looks like this:
``` rust
pub fn solve(&mut board) -> Vec<String> {
  let solutions = Vec<string>
  for x in 0..mx {
    for y in 0..my {
      let mut possibles = Scanned::new("".to_string(), Vec::new());
      solveforpos(board, x, y, &mut possibles, &mut solutions);
    }
  }
  solutions.sort();
  solutions.dedup();
  solutions.to_vec()
}
```
Step 2. is `solveforpos()`, where we implement the recursive strategy.
But for one technicality, we could have implemented this as a single
function, but for one little detail: The letter 'Q'.  There are special
rules about Q, and we have to support them, as the kids say, "because
English."
``` rust
pub(in crate) fn solveforpos(
  board: &Board, (x, y): (isize, isize),
  curr: &mut Scanned, solutions: &mut Vec<String>)
) {
  let c = board.board[x as usize][y as usize];
  innersolveforpos(c, board, (x, y), curr, solutions, false);
  if c == 'q' {
    innersolveforpos('u', board, (x, y), curr, solutions, true);
  }
}
```
The `innersolveforpos()` function checks for word validity, prefix
validity, and position validity.  Not shown here (but you can find it in
the source), the `Scanned` object actually has the responsibility for
adding the letter *if* the position is valid, and returning a new,
longer "maybe-word" (remember, we have to support backtracking in our
recursion) and a new, updated path Ledger.  So that's where we pass the
"skip position check" flag, which in turn lets us put both "Q" (so we
catch words like 'sheqel' and 'burqa') and "Qu" letters into our
candidate string.

Look above, and you'll see that we add 'qu' *blindly* whenever we
encounter 'q'.  This is important.  We have to let that happen because
we need to continue even if "qe" and "qa" aren't in the candidate list.
"Quota" is a real word.

Once we've added the letter(s) and determined that the string we have is
the prefix of a word found in the dictionary, we then scan the neighbors
and recurse, skipping the current cube.  The Ledger makes sure that we
don't re-check a letter for a given single search, but by cloning the
letter and the candidate we also ensure that the backtracking is done
correctly.
``` rust
fn innersolveforpos(c: char, board: &Board, (x, y): (isize, isize),
  curr: &mut Scanned, solutions: &mut Vec<String>, skip_pos_check: bool
) {
  match curr.add(c, (x, y), skip_pos_check) {
    None => return,
    Some(mut newcurr) => {
      if newcurr.0.len() > 2 && board.words.find(&mut newcurr.0.chars()) {
        solutions.push(newcurr.0.to_string());
      }
      if !board.words.pref(&mut newcurr.0.chars()) {
        return;
      }

      for i in -1..=1 {
        for j in -1..=1 {
          if !(i == 0 && j == 0) {
            // Skip the current block!
            let (nx, ny): (isize, isize) = (x as isize + i, y as isize + j);
            if nx >= 0 && nx < board.mx && ny >= 0 && ny < board.my {
              solveforpos(board, (nx, ny), &mut newcurr, solutions)
            }
          }
        }
      }
    }
  }
}
```
And that's pretty much how you solve Boggle.  According to [one
source](http://www.danvk.org/wp/2007-08-02/how-many-boggle-boards-are-there/),
the total number of Boggle boards out there in (nxm)! (that's the
[factorial](https://en.wikipedia.org/wiki/Factorial) symbol there), or for 4x4 board, 16!,
or it would take 20,922,789,888,000 visits to do absolutely every search
of the board.  *Except* for one thing: the English language is not
random!  It's messy, but not random.  The fact that many letter
combinations cannot actually lead to a real word found in the candidate
dictionary means that the vast majority of searches terminate early.

On my laptop, a 4x4 board with all 'e's and a dictionary of 'eee'
through 'eeeeeeeeeeeeeeee' takes 5 minutes and 45 seconds to complete.
But in practice, the average runtime of a boggle board with this
algorithm is barely 1.5 milliseconds.

Which is not too damn bad at all.

Can we go faster?  Yes we can.
