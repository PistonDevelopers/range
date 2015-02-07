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

    /// Returns true if range is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the next offset
    pub fn next_offset(&self) -> usize {
        self.offset + self.length
    }
}

