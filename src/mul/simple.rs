
//! Simple multiplication algorithm.

use crate::{
    arch::{
        self,
        word::{SignedWord, Word},
    },
    memory::{self, Memory},
    mul::{self, helpers},
    sign::Sign::{self, *},
};
use alloc::alloc::Layout;

/// Split larger length into chunks of CHUNK_LEN..2 * CHUNK_LEN for memory locality.
const CHUNK_LEN: usize = 1024;

/// Max supported smaller factor length.
pub(crate) const MAX_SMALLER_LEN: usize = CHUNK_LEN;

/// Temporary memory required for multiplication.
///
/// n bounds the length of the smaller factor in words.
pub(crate) fn memory_requirement_up_to(_n: usize) -> Layout {
    memory::zero_layout()
}

/// c += sign * a * b