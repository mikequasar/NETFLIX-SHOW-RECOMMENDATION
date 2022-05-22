//! Multiplication.

use crate::{
    add,
    arch::word::{SignedWord, Word},
    memory::Memory,
    primitive::{double_word, extend_word, split_double_word},
    sign::Sign,
};
use alloc::alloc::Layout;
use core::mem;
use static_assertions::const_assert;

/// If smaller length <= MAX_LEN_SIMPLE, simple multiplication can be used.
const MAX_LEN_SIMPLE: usize = 24;
const_assert!(MAX_LEN_SIMPLE <= simple::MAX_SMALLER_LEN);
const_assert!(MAX_LEN_SIMPLE + 1 >= karatsuba::MIN_LEN);

/// If smaller length <= this, Karatsuba multiplication can be used.
const MAX_LEN_KARATSUBA: usize = 192;
const_assert!(MAX_LEN_KARATSUBA + 1 >= toom_3::MIN_LEN);

mod helpers;
mod karatsuba;
pub(crate) mod ntt;
mod simple;
mod toom_3;

/// Multiply a word sequence by a `Word` in place.
///
/// Returns carry.
#[must_use]
pub(crate) fn mul_word_in_place(words: &mut [Word], rhs: Word) -> Word {
    mul_word_in_place_with_carry(words, rhs, 0)
}

/// Multiply a word sequence by a `Word` in place with carry in.
///
/// Returns carry.
#[must_use]
pub(crate) fn mul_word_in_place_with_carry(words: &mut [Word], rhs: Word, mut carry: Word) -> Word {
    for a in words {
        // a * b + carry <= MAX * MAX + MAX < DoubleWord::MAX
        let (v_lo, v_hi) =
            split_double_word(extend_word(*a) * extend_word(rhs) + extend_word(carry));
        *a = v_lo;
        carry = v_hi;
    }
    carry
}

/// words += mult * rhs
///
/// Returns carry.
#[must_use]
fn add_mul_word_same_len_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() == rhs.len());
    let mut carry: Word = 0;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        // a + mult * b + carry <= MAX * MAX + 2 * MAX <= DoubleWord::MAX
        let (v_lo, v_hi) = split_double_word(
            extend_word(*a) + extend_word(carry) + extend_word(mult) * extend_word(*b),
        );
        *a = v_lo;
        carry = v_hi;
    }
    carry
}

/// words += mult * rhs
///
/// Returns carry.
#[must_use]
fn add_mul_word_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() >= rhs.len());
    let n = rhs.len();
    let mut carry = add_mul_word_same_len_in_place(&mut words[..n], mult, rhs);
    if words.len() > n {
        carry = Word::from(add::add_word_in_place(&mut words[n..], carry));
    }
    carry
}

/// words -= mult * rhs
///
/// Returns borrow.
#[must_use]
pub(crate) fn sub_mul_word_same_len_in_place(words: &mut [Word], mult: Word, rhs: &[Word]) -> Word {
    assert!(words.len() == rhs.len());
    // carry is in -Word::MAX..0
    // carry_plus_max = carry + Word::MAX
    let mut carry_plus_max = Word::MAX;
    for (a, b) in words.iter_mut().zip(rhs.iter()) {
        // Compute val = a - mult * b + carry_plus_max - MAX + (MA