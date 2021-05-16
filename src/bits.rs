
//! Bitwise operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    helper_macros,
    ibig::IBig,
    math,
    ops::{AndNot, NextPowerOfTwo, UnsignedAbs},
    primitive::{double_word, PrimitiveSigned, PrimitiveUnsigned, WORD_BITS_USIZE},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

impl UBig {
    /// Returns true if the `n`-th bit is set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(0b10010).bit(1), true);
    /// assert_eq!(ubig!(0b10010).bit(3), false);
    /// assert_eq!(ubig!(0b10010).bit(100), false);
    /// ```
    #[inline]
    pub fn bit(&self, n: usize) -> bool {
        match self.repr() {
            Small(word) => n < WORD_BITS_USIZE && word & 1 << n != 0,
            Large(buffer) => {
                let idx = n / WORD_BITS_USIZE;
                idx < buffer.len() && buffer[idx] & 1 << (n % WORD_BITS_USIZE) != 0
            }
        }
    }

    /// Set the `n`-th bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// let mut a = ubig!(0b100);
    /// a.set_bit(0);
    /// assert_eq!(a, ubig!(0b101));
    /// a.set_bit(10);
    /// assert_eq!(a, ubig!(0b10000000101));
    /// ```
    #[inline]
    pub fn set_bit(&mut self, n: usize) {
        match mem::take(self).into_repr() {
            Small(word) => {
                if n < WORD_BITS_USIZE {
                    *self = UBig::from_word(word | 1 << n)
                } else {
                    *self = UBig::with_bit_word_slow(word, n)
                }
            }
            Large(buffer) => *self = UBig::with_bit_large(buffer, n),
        }
    }

    fn with_bit_word_slow(word: Word, n: usize) -> UBig {
        debug_assert!(n >= WORD_BITS_USIZE);
        let idx = n / WORD_BITS_USIZE;
        let mut buffer = Buffer::allocate(idx + 1);
        buffer.push(word);
        buffer.extend((1..idx).map(|_| 0));
        buffer.push(1 << (n % WORD_BITS_USIZE));
        buffer.into()