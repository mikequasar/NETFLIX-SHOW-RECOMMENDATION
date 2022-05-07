//! Helper functions for multiplication algorithms.

use crate::{
    add,
    arch::word::{SignedWord, Word},
    memory::Memory,
    mul,
    sign::Sign,
};

/// c += sign * a * b
///
/// Splits a into chunks of chunk_len, using regular multiplication for the remainder if any.
///
/// Returns carry.
pub(crate) fn add_signed_mul_split_into_chunks<F>(
    mut c: &mut [Word],
    sign: Sign,
    mut a: &[Word],
    b: &[Word],