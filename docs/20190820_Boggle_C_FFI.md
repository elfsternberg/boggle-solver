In my quest to become the world's greatest programmer, I have been
informed that I must reboot my React and Django skills.  What better way
to make that happen than to port my [Boggle solver
program](https://github.com/elfsternberg/boggle-solver) to the web?

It's not good enough to shell out to `boggle-solve` and just make things
happen.  I wanted something that required a little more learning.  And
therefore, we come to the next step in the evolution of the boggle
solving library: A C foreign function interface.

The good part of this story is that we're still going to be writing
almost entirely in Rust.  As long as we stay away from asynchronous
Rust, the Rust runtime can sit comfortably next to the C runtime without
a headache.  (And yes, C has a runtime, but there are historical reasons
for why that runtime is provided by the OS rather than by the compiler.)

There is one catch.  The existing boggle solver takes a dictionary and
at least one board.  But we can solve many boards with the same
dictionary.  So we must answer this question: *Do we want the dictionary
to be reusable?* Loading the dictionary takes a large amount of the
overall processing time and being able to load the dictionary once and
re-use it would be worthwhile.  The dictionary, however, is a complex
Rust structure with a type paramater.

We could answer the question with 'No.'  In which case our function
takes a path to a dictionary, and a memory blob that represents the
board, and returns another memory blob representing the answers.

That is sad.  We don't do sad things.  Yes, the dictionary needs to be
reusable.

The first thing we're going to do is lock down the dictionary's type.
Consistently, we've been using `Node<char>`.  `Node` is a node in a trie
structure.  It has no specialized root node, so there's no point in
having a `Trie<char>` object.  For the FFI, we want Rust to make sure
the layout of our container is compatible with C.  We'll use the
compiler directive `repr(C)` to do that, and then contain our root node in
a new struct:
```
#[repr(C)]
pub struct Trie(boggle_solver::trie::Node<char>);
```
For the C FFI, the Rust compiler directive `no_mangle` tells Rust not to
change the names of the functions in a Rusty way, but to leave them
as-is so the C runtime, which is primitive and doesn't know anything
about name mangling, will be able to find them.

We'e about to wander into the world of Rust black magic here.  The
Rustinomican, the book about Rust's darker world, has [dire
warnings](https://doc.rust-lang.org/nomicon/transmutes.html) about what
I'm about to show you.  This is C-land, where C-things happen and
[C-corruption](https://www.youtube.com/watch?v=BL2Enp8d1Zk) is the order
of the day.

In the original boggle-solver there is a function, `dict(path: &str) ->
Node<char>` that takes a path to a dictionary file and returns a fully
populated trie.  We're going to use that function, wrap the results in
an [opaque pointer](https://www.geeksforgeeks.org/opaque-pointer/) (a
pointer the C programmer shouldn't examine internally), and return that
pointer.  That pointer has to be taken away from Rust's type handler.
Rust needs to not worry about it, not borrow check it. It is a sinful
object lost to the world of C.  The Rust function `transmute` does
this:
```
#[no_mangle]
unsafe extern "C" fn dictionary_make(filepath: *const c_char) -> *const Trie {
    transmute(Box::new(Trie(dict(
        CStr::from_ptr(filepath).to_str().unwrap(),
    ))))
}
```
This function is marked `unsafe`, which is fine because only C-side
programs will ever call it. Inside, we take the filepath, which is an
array of bytes terminated by a null, and cast it to a Rust string, then
pass that to our dictionary builder, wrap the newly made dictionary in
the new `Trie` struct, `Box` it up, and then `transmute` that box into
our C-like pointer. Phew!

Since we are back in the land of C we are responsible for freeing the
dictionary when we're done with it.  Miraculously, we can get away with
this by using `transmute` to hand it back to Rust, put it into a scope,
and then... just let Rust drop it and recover the memory.
```
#[no_mangle]
unsafe extern "C" fn dictionary_destroy(trie: *const Trie) {
    let _drop_me: Box<Trie> = transmute(trie);
}
```
Now *that* is Rust black magic!

We can now solve a board for a given dictionary.  I won't copy the
entire text of the `boggle-solve` binary here, which makes up the bulk
of this function.  At the end of that function, we had a vector of all
the words found on the board, which was the return type of the function.
In our current case, we need to return something C understands.

Traditionally, the way C does this is to allocate a buffer big enough to
hold the results, pass that buffer to the function (as a pointer), and
then expect the buffer to be full when the results come back.

Here, we take the `solutions: Vec<String>` and from it create a massive
string of all the results separated by newlines (to make it printable).
We cast that string into C, and copy the results into the buffer.  This
isn't the world's most efficient example; at one point, we have *three
copies* of the solution in memory, and the "known highest-scoring board"
returns a solution set that's 4,604 bytes long (including newlines).
Using a conservative allocation scheme means that at some point we're
using 24 kilobytes of memory.  It's only for a few seconds until it
drops all but the 8KB used for the parent buffer. Even in these days of
multi-gigabyte machines, that still seems like a lot:
```
#[no_mangle]
unsafe extern "C" fn solve_for_dictionary(
    board_text: *const c_char,
    dictionary: *const Trie,
    found_words: *mut c_char,
) {

...

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
```
And finally, we can now solve given the path to the dictionary.  All of
these functions are C functions with C signatures.  If we didn't call
`dictionary_destroy()` there, the trie would never be recovered, and
that would be a memory leak.  We're in C land here, where memory
management is once again completely and utterly our responsibility.
```
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
```
To make this work with C, we need to provide a C-style header file that
tells the C compiler the names and signatures of the objects we've
created.  After all the `no_mangle` and our `repr` opaque structure,
here's everything C needs to know about:
```
#include <stdlib.h>

struct Trie;

struct Trie* dictionary_make(char* filepath);
void dictionary_destroy(struct Trie* trie);
void solve_for_dictionary(char* board_text, struct Trie* dictionary, char* buffer);
void solve(char* board_text, char* dictionary_filepath, char* buffer);
```
A quick (and very no-frills; I've removed ALL the error-handling
regarding arguments and board reading) example of using this in C:
```
#include "boggle_solver.h"
#include <stdio.h>
#include <string.h>

#define MAXBOARDSIZE 64
#define MAXRETURNSIZE 8192

int main (int argc, char** argv) {
  char board[MAXBOARDSIZE];
  char buffer[MAXRETURNSIZE];
  FILE *fp;

  fp = fopen(argv[1], "r");
  size_t len = fread(board, sizeof(char), MAXBOARDSIZE, fp);
  board[len++] = '\0'; /* Just to be safe. */
  fclose(fp);

  solve(board, "/usr/share/dict/words", buffer);
  printf("%s\n", buffer);
  return 0;
}
```
To use this code, save this in a file name `csolve.c` in the root directory of the FFI
library.  We can then build the library and the binary with cargo and
GCC:
```
cargo build --release
gcc ./csolve ./csolve.c -Isrc  -L. -l:target/release/libboggle_solver.so
```
We can then run this code against the example board:
```
$ ./example/csolve ../sample_board.txt
aerie
air
ape
...
wore
wren
wrier
```
So now we have boggle-solver, the C-land version. The binary isn't even
very large (about 8K on my Linux box), but everything is dynamically
linked so, in the end, it's a big executable, hauling the entire Rust
runtime with it.

Still, this is a pretty good demonstration of what's possible.  It's
not a *great* demonstration; in that case, I'd replace the call to
`dict` with something which could take arrays of strings, or maybe even
a file pointer, and do magic with it inside of Rust.  But this
accomplishes the initial goal: get something that can be run from a C
program and run the Boggle Solver.

Next: Go one step further, and make it work in Python.
