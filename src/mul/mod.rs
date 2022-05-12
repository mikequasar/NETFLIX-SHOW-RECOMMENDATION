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
mod simp