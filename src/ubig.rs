//! Unsigned big integer.

use self::Repr::*;
use crate::{
    arch::{ntt, word::Word},
    buffer::Buffer,
    math,
    primitive::WORD_BITS_USIZE,
};
use core::slice;

/// Internal representation of UBig.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum Repr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    ///
    /// The buffer has:
    /// * length at least 2
    /// * no leading zero
    /// * compact capacity
    Large(Buffer),
}

/// Unsigned big integer.
///
/// Arbitrarily large unsigned integer.
///
/// # Examples
///
/// ```
/// # use ibig::{error::ParseError, ubig, UBig};
/// let a = ubig!(a2a123bbb127779cccc123123ccc base 32);
/// let b = ubig!(0x1231abcd4134);
/// let c = UBig::from_str_radix("a2a123bbb127779cccc123123ccc", 32)?;
/// let d = UBig::from_str_radix("1231abcd4134", 16)?;
/// assert_eq!(a, c);
/// assert_eq!(b, d);
/// # Ok::<(), ParseError>(())
/// ```
#[derive(Eq, Hash, PartialEq)]
pub struct UBig(Repr);

impl UBig {
    /// Construct from one word.
    #[inline]
    pub(crate) fn from_word(word: Word) -> UBig {
        UBig(Small(word))
    }

    /// Get the representation of UBig.
    #[inline]
    pub(crate) fn repr(&self) -> &Repr {
        &self.0
    }

    /// Convert into representation.
    #[inline]
    pub(crate) fn into_repr(self) -> Repr {
        self.0
    }

    /// Length in Words.
    #[inline]
    pub(crate) fn len(&self) -> usize {
        match self.repr() {
            Small(_) => 1,
            Large(buffer) => buffer.len(),
        }
    }

    /// Representation in Words.
    #[inline]
    pub(crate) fn as_words(&self) -> &[Word] 