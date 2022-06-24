
//! Primitive integral types.

use crate::{
    arch::word::{DoubleWord, Word},
    error::OutOfBoundsError,
    sign::Sign::{self, *},
};
use core::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    mem,
    ops::{Add, Div, Mul, Shl, Shr, Sub},
};

/// Cast `Word` to `DoubleWord`.
#[inline]
pub(crate) const fn extend_word(word: Word) -> DoubleWord {
    word as DoubleWord
}

/// Create a `DoubleWord` from two `Word`s.
#[inline]
pub(crate) const fn double_word(low: Word, high: Word) -> DoubleWord {
    extend_word(low) | extend_word(high) << WORD_BITS
}

#[inline]
pub(crate) const fn split_double_word(dw: DoubleWord) -> (Word, Word) {
    (dw as Word, (dw >> WORD_BITS) as Word)
}

pub(crate) trait PrimitiveUnsigned
where
    Self: Copy,
    Self: Debug,
    Self: Default,
    Self: From<u8>,
    Self: TryFrom<Word>,
    Self: TryInto<Word>,
    Self: TryInto<usize>,
    Self: Eq,
    Self: Add<Output = Self>,
    Self: Div<Output = Self>,
    Self: Mul<Output = Self>,
    Self: Sub<Output = Self>,
    Self: Shl<u32, Output = Self>,
    Self: Shr<u32, Output = Self>,
{
    const BYTE_SIZE: usize = mem::size_of::<Self>();