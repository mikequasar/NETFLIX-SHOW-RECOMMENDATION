
//! Mathematical functions.

use crate::{arch::word::Word, assert::debug_assert_in_const_fn, primitive::PrimitiveUnsigned};

/// The length of an integer in bits.
/// 0 for 0.
#[inline]
pub(crate) fn bit_len<T: PrimitiveUnsigned>(x: T) -> u32 {
    T::BIT_SIZE - x.leading_zeros()
}

/// The length of an integer in bits.
/// 0 for 0.
#[inline]
pub(crate) const fn bit_len_word(x: Word) -> u32 {
    Word::BIT_SIZE - x.leading_zeros()
}

/// Ceiling of log_2(x).
/// x must be non-zero.
#[inline]
pub(crate) fn ceil_log_2<T: PrimitiveUnsigned>(x: T) -> u32 {
    debug_assert!(x != T::from(0u8));
    bit_len(x - T::from(1u8))
}

/// Ceiling of log_2(x).
/// x must be non-zero.
#[inline]
pub(crate) const fn ceil_log_2_word(x: Word) -> u32 {
    debug_assert_in_const_fn!(x != 0);
    bit_len_word(x - 1)
}

/// Ceiling of a / b.
#[inline]
pub(crate) fn ceil_div<T: PrimitiveUnsigned>(a: T, b: T) -> T {
    if a == T::from(0u8) {
        T::from(0u8)
    } else {
        (a - T::from(1u8)) / b + T::from(1u8)
    }
}

/// Ceiling of a / b.
#[inline]
pub(crate) const fn ceil_div_usize(a: usize, b: usize) -> usize {
    if a == 0 {
        0
    } else {
        (a - 1) / b + 1
    }
}

/// Round up a to a multiple of b.
#[inline]
pub(crate) fn round_up<T: PrimitiveUnsigned>(a: T, b: T) -> T {
    ceil_div(a, b) * b
}

/// Round up a to a multiple of b.
#[inline]
pub(crate) const fn round_up_usize(a: usize, b: usize) -> usize {
    ceil_div_usize(a, b) * b
}

/// n ones: 2^n - 1
#[inline]
pub(crate) fn ones<T>(n: u32) -> T
where
    T: PrimitiveUnsigned,
{
    if n == 0 {
        T::from(0u8)
    } else {
        T::MAX >> (T::BIT_SIZE - n)
    }
}

/// n ones: 2^n - 1
#[inline]
pub(crate) const fn ones_word(n: u32) -> Word {
    if n == 0 {
        0
    } else {
        Word::MAX >> (Word::BIT_SIZE - n)
    }
}

#[inline]
pub(crate) const fn min_usize(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]