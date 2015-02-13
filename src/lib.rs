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
    #[inline(always)]
    pub fn new(offset: usize, length: usize) -> Range {
        Range {
            offset: offset,
            length: length,
        }
    }

    /// Creates an empty range with an offset.
    #[inline(always)]
    pub fn empty(offset: usize) -> Range {
        Range {
            offset: offset,
            length: 0,
        }
    }

    /// Returns true if range is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the next offset
    #[inline(always)]
    pub fn next_offset(&self) -> usize {
        self.offset + self.length
    }

    /// Returns a range iterator.
    #[inline(always)]
    pub fn iter(&self) -> std::ops::Range<usize> {
        self.offset..self.offset + self.length
    }

    /// Shrinks range at both ends with `n` items.
    #[inline(always)]
    pub fn shrink_n(&self, n: usize) -> Option<Range> {
        if self.length < 2 * n {
            None
        } else {
            Some(Range::new(self.offset + n, self.length - 2 * n))
        }
    }

    /// Shrinks range at both ends with 1 item.
    #[inline(always)]
    pub fn shrink(&self) -> Option<Range> {
        self.shrink_n(1)
    }
}

/// The parent/child relationship for hierarchial contiguous arrays.
/// Meant to be used by newtypes wrapping `Range` for type safety.
pub trait ParentRange {
    type Child;

    /// Creates parent range from inner range.
    fn from_range(range: Range) -> Self;
    /// Gets the immutable inner range.
    fn range(&self) -> &Range;
    /// Gets the mutable inner range.
    fn range_mut(&mut self) -> &mut Range;
}

