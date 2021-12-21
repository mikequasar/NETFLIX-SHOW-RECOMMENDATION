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
        //
        // Note:
        // divisor * (B + m) = divisor * floor(B * 2^n / divisor + 1)
        // = B * 2^n + k, 1 <= k <= divisor

        // m = floor(B * (2^n-1 - (divisor-1)) / divisor) + 1
        let (lo, _hi) = split_double_word(
            double_word(0, math::ones_word(n) - (divisor - 1)) / extend_word(divisor),
        );
        // assert!(_hi == 0);
        FastDivideSmall {
            divisor,
            shift: n - 1,
            m: lo + 1,
        }
    }

    /// ( a / divisor, a % divisor)
    #[inline]
    pub(crate) fn div_rem(&self, a: Word) -> (Word, Word) {
        // q = floor( (B + m) * a / (B * 2^n) )
        //
        // Remember that divisor * (B + m) = B * 2^n + k, 1 <= k <= 2^n
        //
        // (B + m) * a / (B * 2^n)
        // = a / divisor * (B * 2^n + k) / (B * 2^n)
        // = a / divisor + k * a / (divisor * B * 2^n)
        // On one hand, this is >= a / divisor
        // On the other hand, this is:
        // <= a / divisor + 2^n * (B-1) / (2^n * B) / divisor
        // < (a + 1) / divisor
        //
        // Therefore the floor is always the exact quotient.

        // t = m * n / B
        let (_, t) = split_double_word(extend_word(self.m) * extend_word(a));
        // q = (t + a) / 2^n = (t + (a - t)/2) / 2^(n-1)
        let q = (t + ((a - t) >> 1)) >> self.shift;
        let r = a - q * self.divisor;
        (q, r)
    }

    #[inline]
    pub(crate) const fn dummy() -> Self {
        FastDivideSmall {
            divisor: 0,
            shift: 0,
            m: 0,
        }
    }
}

/// Divide a DoubleWord by a prearranged divisor.
///
/// Assumes quotient fits in a Word.
///
/// Möller, Granlund, "Improved division by invariant integers"
/// Algorithm 4.
#[derive(Clone, Copy)]
pub(crate) struct FastDivideNormalized {
    // Top bit must be 1.
    divisor: Word,

    // floor ((B^2 - 1) / divisor) - B, where B = 2^WORD_BITS
    m: Word,
}

impl FastDivideNormalized {
    /// Initialize from a given normalized divisor.
    ///
    /// divisor must have top bit of 1
    #[inline]
    pub(crate) const fn new(divisor: Word) -> Self {
        assert_in_const_fn(divisor.leading_zeros() == 0);
        let (m, _hi) = split_double_word(DoubleWord::MAX / extend_word(divisor));
        assert_in_const_fn(_hi == 1);

        // Note:
        // m > 0
        // (m + B) * divisor = B^2 - k for some 1 <= k <= divisor

        FastDivideNormalized { divisor, m }
    }

    #[inline]
    pub(crate) const fn div_rem_word(&self, a: Word) -> (Word, Word) {
        if a < self.divisor {
            (0, a)
        } else {
            (1, a - self.divisor)
        }
    }

    /// (a / divisor, a % divisor)
    /// The result must fit in a single word.
    #[inline]
    pub(crate) const fn div_rem(&self, a: DoubleWord) -> (Word, Word) {
        let (a_lo, a_hi) = split_double_word(a);
        debug_assert_in_const_fn!(a_hi < self.divisor);

        // Approximate quotient is (m + B) * a / B^2 ~= (m * a/B + a)/B.
        // This is q1 below.
        // This doesn't overflow because a_hi < Word::MAX.
        let (q0, q1) = split_double_word(extend_word(self.m) * extend_word(a_hi) + a);

        // q = q1 + 1 is our first approximation, but calculate mod B.
        // r = a - q * d
        let q = q1.wrapping_add(1);
        let r = a_lo.wrapping_sub(q.wrapping_mul(self.divisor));

        // Theorem: max(-d, q0+1-B) <= r < max(B-d, q0)
        // Proof:
        // r = a - q * d = a - q1 * d - d
        // = a - (q1 * B + q0 - q0) * d/B - d
        // = a - (m * a_hi + a - q0) * d/B - d
        // = a - ((m+B) * a_hi + a_lo - q0) * d/B - d
        // = a - ((B^2-k)/d * a_hi + a_lo - q0) * d/B - d
        // = a - B * a_hi + (a_hi * k - a_lo * d + q0 * d) / B - d
        // = (a_hi * k + a_lo * (B - d) + q0 * d) / B - d
        //
        // r >= q0 * d / B - d
        // r >= -d
        // r >= d/B (q0 - B) > q0-B
        // r >= max(-d, q0+1-B)
        //
        // r < (d * d + B * (B-d) + q0 * d) / B - d
        // = (B-d)^2 / B + q0 * d / B
        // = (1 - d/B) * (B-d) + (d/B) * q0
        // <= max(B-d, q0)
        // QED

        // if r mod B > q0 { q -= 1; r += d; }
        //
        // Consider two cases:
 