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
    chunk_len: usize,
    memory: &mut Memory,
    f_add_signed_mul_chunk: F,
) -> SignedWord
where
    F: Fn(&mut [Word], Sign, &[Word], &[Word], &mut Memory) -> SignedWord,
{
    debug_assert!(a.len() >= b.len() && c.len() == a.len() + b.len());
    debug_assert!(b.len() <= chunk_len);

    let n = b.len();
    let mut carry_n = 0; // at c[n]
    while a.len() >= chunk_