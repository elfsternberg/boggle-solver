use fsbitmap::FSBitmap;

/// This is a simple ledger for the parts of the board that have been
/// visited during the search process.  It hosts the dimensions of the
/// board and a bitmap.  This implementation uses the
/// Fast-Simple-Bitmap, which is itself backed by a Vec.
///
pub struct Ledger(isize, isize, FSBitmap);

impl Ledger {
    pub fn new(height: isize, width: isize) -> Ledger {
        Ledger(height, width, FSBitmap::new((height * width) as usize))
    }

    #[inline]
    fn next(&self, ledger: FSBitmap) -> Ledger {
        Ledger(self.0, self.1, ledger)
    }

    #[inline]
    fn point(&self, x: isize, y: isize) -> u64 {
        (x * self.1 + y) as u64
    }

    /// Generates a *new* ledger, marks the point requested, and returns
    /// the Ledger.
    ///
    /// This allows for fast backtracking at the expense of some
    /// memory. At no point are there more than height*width ledgers
    /// active, so the memory usage isn't terrible.
    ///
    pub fn mark(&mut self, x: isize, y: isize) -> Ledger {
        let mut newmap = self.2.clone();
        newmap.mark(self.point(x, y) as usize);
        self.next(newmap)
    }

    /// Checks if the bit is set.
    pub fn check(&self, x: isize, y: isize) -> bool {
        self.2.check(self.point(x, y) as usize)
    }
}
