//! Divide by a prearranged Word quickly using multiplication by the reciprocal.

use crate::{
    arch::word::{DoubleWord, Word},
    assert::{assert_in_const_fn, debug_assert_in_const_fn},
    math,
    primitive::{double_word, extend_word, split_double_word},
};

/// Divide a Word by a prearranged divisor.
///
/// Granlund, Montgomerry "Division by Invariant Integers using Multiplication"
/// Algorithm 4.1.
#[derive(Clone, Copy)]
pub(crate) struct FastDivideSmall {
    // 2 <= divisor < 2^N, N = WORD_BITS
    divisor: Word,

    // Let n = ceil(log_2(divisor))
    // 2^(n-1) < divisor <= 2^n
    // shift = n - 1
    shift: u32,

    // m = floor(B * 2^n / divisor) + 1 - B, where B = 2^N
    m: Word,
}

impl FastDivideSmall {
    #[inline]
    pub(crate) const fn new(divisor: Word) -> Self {
        assert_in_const_fn(divisor > 1);
        let n = math::ceil_log_2_word(divisor);

        // Calculate:
        // m = floor(B * 2^n / divisor) + 1 - B
        // m >= B + 1 - B >= 1
        // m <= B * 2^n / (2^(n-1) + 1) + 1 - B
        //    = (B * 2^n + 2^(n-1) + 1) / (2^(n-1) + 1) - B
        //    = B * (2^n + 2^(n-1-N) + 2^-N) / (2^(n-1)+1) - B
        //    < B * (2^n + 2^1) / (2^(n-1)+1) - B
        //    = B
        // So m fits in a Word.
  