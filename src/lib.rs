#![deny(missing_docs)]

//! A library for range addressing

/// A representation of a range
///
/// The type parameter is used to wrap data related to the range.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Range<T = ()> {
    /// The range offset
    pub offset: usize,
    /// The range length
    pub length: usize,
    /// The data described within the range.
    pub data: T,
}

impl Range {
    /// Creates a new `Range`
    #[inline(always)]
    pub fn new(offset: usize, length: usize) -> Range {
        Range {
            offset: offset,
            length: length,
            data: (),
        }
    }

    /// Wraps some data in the range.
    #[inline(always)]
    pub fn wrap<T>(self, data: T) -> Range<T> {
        Range {
            offset: self.offset,
            length: self.length,
            data: data,
        }
    }

    /// Creates an empty range with an offset.
    #[inline(always)]
    pub fn empty(offset: usize) -> Range {
        Range {
            offset: offset,
            length: 0,
            data: (),
        }
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

    /// Intersects a range with another, where ends are excluded.
    pub fn intersect(&self, other: &Range) -> Option<Range> {
        use std::cmp::{ min, max };

        if other.next_offset() <= self.offset ||
           other.offset >= self.next_offset() {
            None
        } else {
            let offset = max(self.offset, other.offset);
            let length = min(self.next_offset(), other.next_offset())
                - offset;
            Some(Range::new(offset, length))
        }
    }

    /// Intersects a range with another, where ends are included.
    pub fn ends_intersect(&self, other: &Range) -> Option<Range> {
        use std::cmp::{ min, max };

        if other.next_offset() < self.offset ||
           other.offset > self.next_offset() {
            None
        } else {
            let offset = max(self.offset, other.offset);
            let length = min(self.next_offset(), other.next_offset())
                - offset;
            Some(Range::new(offset, length))
        }
    }
}

impl<T> Range<T> {
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

    /// Unwraps the data.
    #[inline(always)]
    pub fn unwrap(self) -> T {
        self.data
    }

    /// Decouples range and data.
    #[inline(always)]
    pub fn decouple(self) -> (Range, T) {
        (Range { offset: self.offset, length: self.length, data: () },
         self.data)
    }
}

impl<T> From<(Range, T)> for Range<T> {
    #[inline(always)]
    fn from((r, d): (Range, T)) -> Range<T> {
        Range {
            offset: r.offset,
            length: r.length,
            data: d
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let a: Range = Range::new(2, 5);
        let b = Range::new(5, 3);
        let c = a.intersect(&b);
        assert_eq!(c, Some(Range::new(5, 2)));
    }

    #[test]
    fn ends_intersect() {
        let a: Range = Range::new(2, 3);
        let b = Range::new(5, 3);
        let c = a.ends_intersect(&b);
        assert_eq!(c, Some(Range::new(5, 0)));
        let c = b.ends_intersect(&a);
        assert_eq!(c, Some(Range::new(5, 0)));
    }
}
