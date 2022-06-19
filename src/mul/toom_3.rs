
//! Toom-Cook-3 multiplication algorithm.

use crate::{
    add,
    arch::word::{SignedWord, Word},
    div, math,
    memory::{self, Memory},
    mul::{self, helpers},
    shift,
    sign::Sign::{self, *},
};
use alloc::alloc::Layout;

// We must have:
// 2 * (n+2) <= n
// i * n3 + 2 <= (i+1) * n3
// 5 * n3 + 2 <= 2n
// where n3 = floor((n+2)/3)
//
// Verify:
// 2 * n3 <= 2/3 (n+2) = 1/3 (2n + 2) <= n if n >= 2
// i * n3 + 2 <= (i+1) * n3 if n3 >= 2
// 5 * n3 + 2 <= 5/3 (n+2) + 2 = 1/3 (5n + 16) <= 2n if n >= 16
// If n >= 16, then n3 >= (16+2)/3 = 6 >= 2
/// Minimum supported length of the factors.
pub(crate) const MIN_LEN: usize = 16;

/// Temporary memory required for multiplication.
///
/// n bounds the length of the smaller factor in words.
pub(crate) fn memory_requirement_up_to(n: usize) -> Layout {
    // In each level of recursion we use:
    // a_eval: n3 + 1
    // b_eval: n3 + 1
    // c_eval: 2 * (n3 + 1)
    // t1:     2 * (n3 + 1)
    // t2:     2 * (n3 + 1)
    // total: 8 * (n3 + 1)
    //
    // Prove by induction that f(n) <= 4n + 20(log_3 (n-2.5)).
    // Base case, f(3) >= 0, OK.
    // For n > 3:
    // f(n)  = 8*(ceil(n/3)+1) + f(ceil(n/3)+1)
    //      <= 8*(n+5)/3 + 4*(n+5)/3 + 20 log_3 ((n+5)/3-2.5)
    //       = 4n + 20 + 20 log_3 ((n+5)/3-2.5)
    //       = 4n + 20 log_3 (n-2.5)
    //
    // 20 log_3 (n-2.5) <= 20 log_3 n = 20 log_2 n / log_2 3 < 13 log_2 n
    // So we use 4n + 13 ceil log_2 n.
    //
    // Note: the recurence also works when we transition to Karatsuba, because
    // Karatsuba memory requirements are smaller.
    let num_words = 4 * n + 13 * (math::ceil_log_2(n) as usize);
    memory::array_layout::<Word>(num_words)
}

/// c += sign * a * b
/// Toom-Cook-3 method. O(a.len() * b.len()^0.47).
///
/// Returns carry.
#[must_use]
pub(crate) fn add_signed_mul(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    memory: &mut Memory,
) -> SignedWord {
    assert!(a.len() >= b.len() && b.len() >= MIN_LEN && c.len() == a.len() + b.len());

    helpers::add_signed_mul_split_into_chunks(
        c,
        sign,
        a,
        b,
        b.len(),
        memory,
        add_signed_mul_same_len,
    )
}

/// c += sign * a * b
/// Toom-Cook-3 method: O(n^1.47).
///
/// Returns carry.
#[must_use]
pub(crate) fn add_signed_mul_same_len(
    c: &mut [Word],
    sign: Sign,
    a: &[Word],
    b: &[Word],
    memory: &mut Memory,
) -> SignedWord {
    let n = a.len();
    debug_assert!(b.len() == n && c.len() == 2 * n);
    debug_assert!(n >= MIN_LEN);

    // Brent, Zimmermann, Modern Computer Arithmetic 0.5.9, Algorithm 1.4.
    //
    // We evaluate the polynomials A(x) = a0 + a1*x + a2*x^2, B(x) = b0 + b1*x + b2*x^2
    // at points 0, 1, -1, 2, infinity.
    // Multiplying, this gives us values of V(x) = A(x)*B(x) = c0 + c1*x + c2*x^2 + c3*x^3 + c4*x^4
    // at the same points (using 5 recursive multiplications).
    //
    // Then we interpolate the polynomial coefficients, which gives the following formulas:
    // c_0 = V(0)
    // c_1 = V(1) - t1
    // c_2 = t2 - V(0) - V(inf)
    // c_3 = t1 - t2
    // c_4 = V(inf)
    // where:
    // t1 = (3V(0) + 2V(-1) + V(2))/6 - 2V(inf)
    // t2 = (V(1) + V(-1))/2

    // Split into 3 parts. Note: a2, b2 may be shorter.
    let n3 = (n + 2) / 3;
    let n3_short = n - 2 * n3;

    let (a0, a12) = a.split_at(n3);
    let (a1, a2) = a12.split_at(n3);
    let (b0, b12) = b.split_at(n3);
    let (b1, b2) = b12.split_at(n3);

    let mut carry: SignedWord = 0;
    // Accumulate intermediate carries, we will add them at the end.
    let mut carry_c0: SignedWord = 0; // at 2*n3
    let mut carry_c1: SignedWord = 0; // at 3*n3+2
    let mut carry_c2: SignedWord = 0; // at 4*n3+2
    let mut carry_c3: SignedWord = 0; // at 5*n3+2