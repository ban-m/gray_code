#[derive(Debug, Clone, Copy)]
pub struct GrayCode {
    length: u64,
}

impl GrayCode {
    pub fn new(length: u64) -> Self {
        Self { length }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GrayCodeFlip {
    length: u64,
}

impl GrayCodeFlip {
    pub fn new(length: u64) -> Self {
        Self { length }
    }
}

impl std::iter::IntoIterator for GrayCode {
    type Item = u64;
    type IntoIter = GrayCodeIterator;
    fn into_iter(self) -> Self::IntoIter {
        GrayCodeIterator::new(self)
    }
}

impl std::iter::IntoIterator for GrayCodeFlip {
    type Item = usize;
    type IntoIter = GrayCodeFlipIterator;
    fn into_iter(self) -> Self::IntoIter {
        GrayCodeFlipIterator::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct GrayCodeIterator {
    length: u64,
    current_index: u64,
    current_code: u64,
    max_index: u64,
}

impl GrayCodeIterator {
    fn new(GrayCode { length }: GrayCode) -> Self {
        Self {
            length,
            current_code: 1,
            current_index: 0,
            max_index: 1 << length,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GrayCodeFlipIterator {
    length: u64,
    current_index: u64,
    max_index: u64,
}

impl GrayCodeFlipIterator {
    fn new(GrayCodeFlip { length }: GrayCodeFlip) -> Self {
        Self {
            length,
            current_index: 0,
            max_index: 1 << length,
        }
    }
}

impl std::iter::Iterator for GrayCodeIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else if self.current_index < self.max_index {
            let flip_bit_location = self.current_index.max(1).trailing_zeros();
            self.current_code = self.current_code ^ (1 << flip_bit_location);
            self.current_index += 1;
            Some(self.current_code)
        } else {
            None
        }
    }
}

impl std::iter::Iterator for GrayCodeFlipIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else if self.current_index < self.max_index {
            let flip_bit_location = self.current_index.max(1).trailing_zeros();
            self.current_index += 1;
            Some(flip_bit_location as usize)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn zero_dig() {
        let mut gc = GrayCode::new(0).into_iter();
        assert_eq!(gc.next(), None);
    }
    #[test]
    fn one_dig() {
        let mut gc = GrayCode::new(1).into_iter();
        assert_eq!(gc.next(), Some(0b0));
        assert_eq!(gc.next(), Some(0b1));
        assert_eq!(gc.next(), None);
    }
    #[test]
    fn two_deg() {
        let mut gc = GrayCode::new(2).into_iter();
        assert_eq!(gc.next(), Some(0b00));
        assert_eq!(gc.next(), Some(0b01));
        assert_eq!(gc.next(), Some(0b11));
        assert_eq!(gc.next(), Some(0b10));
        assert_eq!(gc.next(), None);
    }
    #[test]
    fn three_dig() {
        let mut gc = GrayCode::new(3).into_iter();
        assert_eq!(gc.next(), Some(0b000));
        assert_eq!(gc.next(), Some(0b001));
        assert_eq!(gc.next(), Some(0b011));
        assert_eq!(gc.next(), Some(0b010));
        assert_eq!(gc.next(), Some(0b110));
        assert_eq!(gc.next(), Some(0b111));
        assert_eq!(gc.next(), Some(0b101));
        assert_eq!(gc.next(), Some(0b100));
        assert_eq!(gc.next(), None);
    }
    #[test]
    fn three_dig_iter() {
        let mut gc = GrayCodeFlip::new(3).into_iter();
        assert_eq!(gc.next(), Some(0)); // This behavior might change in later versions.
        assert_eq!(gc.next(), Some(0));
        assert_eq!(gc.next(), Some(1));
        assert_eq!(gc.next(), Some(0));
        assert_eq!(gc.next(), Some(2));
        assert_eq!(gc.next(), Some(0));
        assert_eq!(gc.next(), Some(1));
        assert_eq!(gc.next(), Some(0));
        assert_eq!(gc.next(), None);
    }
    #[test]
    fn large_test() {
        let digit = 10i32;
        let mut bucket = vec![false; 1usize << digit];
        let mut prev: Option<u64> = None;
        for digit in GrayCode::new(digit as u64) {
            if let Some(prev) = prev {
                assert_eq!((prev ^ digit).count_ones(), 1);
            }
            prev = Some(digit);
            assert!(!bucket[digit as usize]);
            bucket[digit as usize] = true;
        }
        assert!(bucket.into_iter().all(|x| x));
    }
    #[test]
    fn large_test_2() {
        let digit = 22i32;
        let mut bucket = vec![false; 1usize << digit];
        let mut prev: Option<u64> = None;
        for digit in GrayCode::new(digit as u64) {
            if let Some(prev) = prev {
                assert_eq!((prev ^ digit).count_ones(), 1);
            }
            prev = Some(digit);
            assert!(!bucket[digit as usize]);
            bucket[digit as usize] = true;
        }
        assert!(bucket.into_iter().all(|x| x));
    }
}
