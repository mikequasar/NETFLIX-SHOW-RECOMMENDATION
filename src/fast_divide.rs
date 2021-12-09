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
#[de