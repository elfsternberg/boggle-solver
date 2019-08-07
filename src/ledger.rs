/// This is a simple ledger for the parts of the board that have been
/// visited during the search process.  It hosts the dimensions of the
/// board and a bitmap.  This only works for boards where the height *
/// width is less than 64, as it uses a 64-bit value as its backing
/// store.
///
pub struct Ledger(isize, isize, u64);

impl Ledger {
    pub fn new(x: isize, y: isize) -> Ledger {
        Ledger(x, y, 0)
    }

    #[inline]
    fn next(&self, ledger: u64) -> Ledger {
        Ledger(self.0, self.1, ledger)
    }

    #[inline]
    fn point(&self, x: isize, y: isize) -> u64 {
        1 << (self.1 * x + y)
    }

    /// Generates a *new* ledger, marks the point requested, and returns
    /// the Ledger.
    ///
    /// This allows for fast backtracking at the expense of some
    /// memory. At no point are there more than height*width ledgers
    /// active, so the memory usage isn't terrible.
    ///
    pub fn mark(&self, x: isize, y: isize) -> Ledger {
        self.next(self.2 | self.point(x, y))
    }

    /// Checks if the bit is set.
    ///
    pub fn check(&self, x: isize, y: isize) -> bool {
        self.2 & self.point(x, y) != 0
    }
}
