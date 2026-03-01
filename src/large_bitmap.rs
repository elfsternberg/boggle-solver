pub struct FSBitmap(Vec<u8>, usize);

impl FSBitmap {
    /// Creates a new FSBitmap of a set size of bits.
    pub fn new(b: usize) -> FSBitmap {
        let s = b / 8 + { if (b % 8) == 0 { 0 } else { 1 } };
        FSBitmap((0..s).map(|_| 0 as u8).collect::<Vec<u8>>(), b)
    }

    /// Clones a copy of the bitmap.
    pub fn clone(&self) -> FSBitmap {
        FSBitmap(self.0.to_vec(), self.1)
    }

    #[inline]
    fn index(&self, p: usize) -> (usize, usize) {
        (p / 8, p % 8)
    }

    /// Mark a given bit.
    ///
    /// Note: This function will panic if you ask to mark a bit outside of the
    /// original size of the bitmap.
    pub fn mark(&mut self, p: usize) {
        let (cell, byte) = self.index(p);
        self.0[cell] |= 1 << byte;
    }

    /// Unmark a given bit.  This function does not panic.
    pub fn unmark(&mut self, p: usize) {
        if p <= self.1 {
            let (cell, byte) = self.index(p);
            self.0[cell] &= !1 << byte;
        }
    }

    /// Flip a given bit: 0 -> 1 or 1 -> 0
    ///
    /// Note: This function will panic if you ask to mark a bit outside of the
    /// original size of the bitmap.
    pub fn flip(&mut self, p: usize) {
        let (cell, byte) = self.index(p);
        self.0[cell] ^= 1 << byte
    }

    /// Returns true if a given bit is set.  This function never panics:
    /// bits outside its range are regarded as never set.
    pub fn check(&self, p: usize) -> bool {
        if p >= self.1 {
            return false;
        }
        let (cell, byte) = self.index(p);
        let v = 1 << byte;
        self.0[cell] & v == v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build() {
        let mut bitmap = FSBitmap::new(127);
        let mut sample: Vec<usize> = vec![41, 23, 79, 63, 65, 64];
        for i in sample.iter() {
            bitmap.mark(*i)
        }
        sample.sort();
        let rng = 0..127;
        let res = rng.filter(|n| bitmap.check(*n)).collect::<Vec<usize>>();
        assert_eq!(res, sample);
    }

    #[test]
    fn can_clone() {
        let mut original_map = FSBitmap::new(127);
        let mut sample: Vec<usize> = vec![41, 23, 79, 63, 65, 64];
        for i in sample.iter() {
            original_map.mark(*i)
        }

        let mut cloned_map = original_map.clone();

        sample.sort();

        // Assert the two maps are identical.
        assert_eq!(
            (0..127)
                .filter(|n| original_map.check(*n))
                .collect::<Vec<usize>>(),
            sample
        );
        assert_eq!(
            (0..127)
                .filter(|n| cloned_map.check(*n))
                .collect::<Vec<usize>>(),
            sample
        );

        // Assert the new map matches the new sample, while the old map is unchanged.
        // Also, make sure the samples really are different.
        cloned_map.unmark(79);
        let newsample = sample
            .iter()
            .filter(|n| **n != 79)
            .map(|n| *n)
            .collect::<Vec<usize>>();

        assert!(sample.len() != newsample.len());
        assert_eq!(
            (0..127)
                .filter(|n| original_map.check(*n))
                .collect::<Vec<usize>>(),
            sample
        );
        assert_eq!(
            (0..127)
                .filter(|n| cloned_map.check(*n))
                .collect::<Vec<usize>>(),
            newsample
        );
    }
}
