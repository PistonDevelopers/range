#![deny(missing_docs)]

//! A library for range addressing

/// A representation of a range
#[derive(Copy, Clone, PartialEq, Eq, Hash, Show)]
pub struct Range {
    /// The range offset
    pub offset: uint,
    /// The range length
    pub length: uint
}

impl Range {
    /// Creates a new `Range`
    pub fn new(offset: uint, length: uint) -> Range {
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
    pub fn next_offset(&self) -> uint {
        self.offset + self.length
    }
}

