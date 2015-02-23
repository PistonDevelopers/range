#![deny(missing_docs)]

//! A library for range addressing

use std::marker::PhantomData;

/// A representation of a range
///
/// The type parameter is used for extra type safety
/// to avoid using a range for the wrong kind of action.
/// This can be a specific action to be performed on the range,
/// for example `Range<AddTo<T>>`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Range<T = ()> {
    /// The range offset
    pub offset: usize,
    /// The range length
    pub length: usize,
    /// Phantom type marker
    phantom: PhantomData<T>,
}

impl<T> Range<T> {
    /// Creates a new `Range`
    #[inline(always)]
    pub fn new(offset: usize, length: usize) -> Range<T> {
        Range {
            offset: offset,
            length: length,
            phantom: PhantomData,
        }
    }

    /// Casts a range into another type.
    #[inline(always)]
    pub fn cast<U>(self) -> Range<U> {
        Range {
            offset: self.offset,
            length: self.length,
            phantom: PhantomData,
        }
    }

    /// Creates an empty range with an offset.
    #[inline(always)]
    pub fn empty(offset: usize) -> Range<T> {
        Range {
            offset: offset,
            length: 0,
            phantom: PhantomData,
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
    pub fn shrink_n(&self, n: usize) -> Option<Range<T>> {
        if self.length < 2 * n {
            None
        } else {
            Some(Range::new(self.offset + n, self.length - 2 * n))
        }
    }

    /// Shrinks range at both ends with 1 item.
    #[inline(always)]
    pub fn shrink(&self) -> Option<Range<T>> {
        self.shrink_n(1)
    }
}

/// Add range to object `T`.
pub struct AddTo<T>(PhantomData<T>);
