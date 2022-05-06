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
/// Splits a into chunks of chunk_len, using regular mu