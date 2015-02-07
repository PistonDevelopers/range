#![deny(missing_docs)]
#![feature(hash)]

//! A library for range addressing

/// A representation of a range
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Range {
    /// The range offset
    pub offset: usize,
    /// The range length
    pub length: usize
}

impl Range {
    /// Creates a new `Range`
    pub fn new(offset: usize, length: usize) -> Range {
        Range {
            offset: offset,
            length: length,
        }
    }

    /// Creates an empty range with an offset.
    pub fn empty(offset: usize) -> Range {
        Range {
            offset: offset,
            length: 0,
        }
    }

    /// Returns true if range is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the next offset
    pub fn next_offset(&self) -> usize {
        self.offset + self.length
    }

    /// Returns a range iterator.
    pub fn iter(&self) -> std::ops::Range<usize> {
        self.offset..self.offset + self.length
    }

    /// Shrinks range at both ends with `n` items.
    pub fn shrink_n(&self, n: usize) -> Option<Range> {
        if self.length < 2 * n {
            None
        } else {
            Some(Range::new(self.offset + n, self.length - 2 * n))
        }
    }

    /// Shrinks range at both ends with 1 item.
    pub fn shrink(&self) -> Option<Range> {
        self.shrink_n(1)
    }
}

