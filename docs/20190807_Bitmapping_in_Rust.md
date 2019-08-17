One of the things I needed in my [Boggle
solver](https://github.com/elfsternberg/boggle-solver) was a bitmap.
The average Boggle! board is 4x4, or 16 squares, and one of the rules is
that you may not use the same letter twice.  I needed a data structure
that would handle the "Have I seen this before?"

Now, for a 4x4 board, a 16-bit integer will work just fine.  And, in
fact, the `Ledger` structure in the solver is, by default, a 64-bit
integer, capable of handling boards as big as 8x8.  In those case, the
"mark" and "check" operations are just indices.  Since this is a
two-dimensional structure, we need a way to convert two-dimensional
constructs into a one-dimensional number.  

We want to move forward through the bitmap, using the width of the
bitmap to find which row we're in, and then the position on that row as
the final offset.

These functions use the Rust bitmap operators.  `<<` means "shift left",
and for a lot of programming operations these are used to mean "multiply
by two."  An accompanying function `>>` means "shift right", and can
mean "divide by two"; in assembly language, both of these functions have
an associated register that will tell you if a zero or one "fell off",
and there are are similar instructions that will actually "shift
around"; if a '1' falls off at the 0th position of a 64-bit register, a
'1' will be inserted into the 63rd position, and vice-versa.

For our purposes, though, we just care about "the bit at a given
position," and we use the shift operator to shift that bit into that
position.  In this case, we just create a new u64 bitmap with that one
bit set:

``` rust
pub struct Ledger(isize, isize, u64);

impl Ledger {
    fn point(&self, x: isize, y: isize) -> u64 {
        1 << (self.1 * x + y)
    }
}
```
And then marking the bitmap consists of 'or'ing it against the
representative bitmap.
``` rust
   fn mark(&self, x: isize, y: isize) {
       self.2 |= self.point(x, y);
   }
```
And checking the bitmap is to 'and' the pointed bitmap against the
internal bitmap and checking that the result is not zero (a very fast
operation in assembly language).
``` rust
   fn check(&self, x: isize, y: isize) -> bool {
       self.2 &= self.point(x, y) != 0
   }
```

As trivial as that is, what if the board is *bigger* than 8x8?  At that
point, we're beyond the largest integer provided by mainstream CPUs, so
we have to move to something simple: a map of maps.  We'll use a
`Vec<u8>`, and kinda do what `point()` does up above, only in reverse:
turn a single coordinate into a coordinate pair indicating which index
in the vector to target, and then which bit in that `u8` we want to set 
or change.

The only challenge here is that we need to know how big the vector will
be ahead of time, and we need to ensure that the vector is pre-populated
and that the entire bitmap starts out set to zero.  In the event that 
our size isn't evenly divisible by eight, we need one more block of bits
for the overflow:
``` rust
pub struct Bitmap(Vec<u8>, usize);

impl Bitmap{
  pub fn new(b: usize) -> FSBitmap {
        let s = b / 8 + { if (b % 8) == 0 { 0 } else { 1 } };
        FSBitmap((0..s).map(|_| 0 as u8).collect::<Vec<u8>>(), b)
  }
}
```
The index of a point is then two numbers: which byte we want to examine,
and then an offset into that byte.  In many ways, it's exactly what
the Ledger does, only backward.  The Ledger doesn't care about machine
sizes, just the board.  The bitmap, though, cares about machine sizes.
``` rust
  fn index(&self, p: usize) -> (usize, usize) {
    (p / 8, p % 8)
  }
```
This format isn't usable *as a number*, but as a way of tracking
"in a modestly large atomic space, which units are in a which binary state?"
it's perfect.  Marking a bit uses this pair:
```
  pub fn mark(&mut self, p: usize) {
    let (cell, byte) = self.index(p);
    self.0[cell] |= 1 << byte;
  }
```
As does checking it.  In this particular case, any bit referenced
outside the size of the requested original size is assumed to be zero:
``` rust
  pub fn check(&self, p: usize) -> bool {
    if p >= self.1 { return false; }
    let (cell, byte) = self.index(p);
    self.0[cell] & (1 << byte) != 0
 }
```
Other operations such as `flip` and `unmark` can be written with just these
operators. 

Bitmaps aren't the most thrilling data structures in the world, although
there are some significant uses for very large, sparse bitmaps that have
to be stored in something more esoteric than just a `Vec<u8>`.  But for
the purposes of Boggle! this was straightforward and not too difficult.
