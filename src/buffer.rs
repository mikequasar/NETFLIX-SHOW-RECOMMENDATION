//! Word buffer.

use crate::{arch::word::Word, ubig::UBig};

use alloc::vec::Vec;
use core::{
    iter,
    ops::{Deref, DerefMut},
};

/// Buffer for Words.
///
/// UBig operations are usually performed by creating a Buffer with appropriate capacity, filling it
/// in with Words, and then converting to UBig.
///
/// If its capacity is exceeded, the `Buffer` will panic.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct Buffer(Vec<Word>);

impl Buffer {
    /// Creates a `Buffer` with at least specified capacity.
    ///
    /// It leaves some extra space for future growth.
    pub(crate) fn allocate(num_words: usize) -> Buffer {
        if num_words > Buffer::MAX_CAPACITY {
            UBig::panic_number_too_large();
        }
        Buffer(Vec::with_capacity(Buffer::default_capacity(num_words)))
    }

    /// Ensure there is enough capacity in the buffer for `num_words`. Will reallocate if there is
    /// not enough.
    #[inline]
    pub(crate) fn ensure_capacity(&mut self, num_words: usize) {
        if num_words > self.capacity() {
            self.reallocate(num_words);
        }
    }

    /// Makes sure that the capacity is compact.
    #[inline]
    pub(crate) fn shrink(&mut self) {
        if self.capacity() > Buffer::max_compact_capacity(self.len()) {
            self.reallocate(self.len());
        }
    }

    /// Change capacity to store `num_words` plus some extra space for future growth.
    ///
    /// # Panics
    ///
    /// Panics if `num_words < len()`.
    fn reallocate(&mut self, num_words: usize) {
        assert!(num_words >= self.len());
        let mut new_buffer = Buffer::allocate(num_words);
        new_buffer.clone_from(self);
        *self = new_buffer
    }

    /// Return buffer capacity.
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Append a Word to the buffer.
    ///
    /// # Panics
    ///
    /// Panics if there is not enough capacity.
    #[inline]
    pub(crate) fn push(&mut self, word: Word) {
        assert!(self.len() < self.capacity());
        self.0.push(word);
    }

    /// Append a Word and reallocate if necessary.
    #[inline]
    pub(crate) fn push_may_reallocate(&mut self, word: Word) {
        self.ensure_capacity(self.len() + 1);
        self.push(word);
    }

    /// Append `n` zeros.
    ///
    /// # Panics
    ///
    /// Panics if there is not enough capacity.
    pub(crate) fn push_zeros(&mut self, n: usize) {
        assert!(n <= self.capacity() - self.len());
        self.0.extend(iter::repeat(0).take(n));
    }

    /// Insert `n` zeros in front.
    ///
    /// # Panics
    ///
    /// Panics if there is not enough capacity.
    pub(crate) fn push_zeros_front(&mut self, n: usize) {
        assert!(n <= self.capacity() - self.len());
        self.0.splice(..0, iter::repeat(0).take(n));
    }

    /// Pop the most significant `Word`.
    #[inline]
    pub(crate) fn pop(&mut self) -> Option<Word> {
        self.0.pop()
    }

    /// Pop leading zero words.
    #[inline]
    pub(crate) fn pop_leading_zeros(&mut self) {
        while let Some(0) = self.last() {
            self.pop();
        }
    }

    #[inline]
    /// Truncate length to `len`.
    pub(crate) fn truncate(&mut self, len: usize) {
        assert!(self.len() >= len);

        self.0.truncate(len);
    }

    /// Erase first n elements.
    pub(crate) fn erase_front(&mut self, n: usize) {
        assert!(self.len() >= n);

        self.0.drain(..n);
    }

    /// Clone from `other` and resize if necessary.
    ///
    /// Equivalent to, but more efficient than:
    ///
    /// ```ignore
    /// buffer.ensure_capacity(source.len());
    /// buffer.clone_from(source);
    /// buffer.shrink();
    /// ```
    pub(crate) fn resizing_clone_from(&mut self, source: &Buffer) {
        let cap = self.capacity();
        let n = source.len();
        if cap >= n && cap <= Buffer::max_compact_capacity(n) {
            self.clone_from(source);
        } else {
            *self = source.clone();
        }
    }

    /// Maximum number of `Word`s.
    ///
    /// We allow 4 extra words beyond `UBig::MAX_LEN` to allow temporary space in operations.
    pub(crate) const MAX_CAPACITY: usize = UBig::MAX_LEN + 4;

    /// Default capacity for a given number of `Word`s.
    /// It should be between `num_words` and `max_capacity(num_words).
    ///
    /// Requires that `num_words <= MAX_CAPACITY`.
    ///
    /// Provides `2 + 0.125 * num_words` extra space.
    #[inline]
    fn default_capacity(num_words: usize) -> usize {
        debug_assert!(num_words <= Buffer::MAX_CAPACITY);
        (num_words + num_words / 8 + 2).min(Buffer::MAX_CAPACITY)
    }

    /// Maximum compact capacity for a given number of `Word`s.
    ///
    /// Requires that `num_wor