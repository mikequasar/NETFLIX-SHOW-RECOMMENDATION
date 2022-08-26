//! Unsigned big integer.

use self::Repr::*;
use crate::{
    arch::{ntt, word::Word},
    buffer::Buffer,
    math,
    primitive::WORD_BITS_USIZE,
};
use core::slice;

/// Internal representation of UBig.
#[derive(Debug, Eq, Hash, PartialEq)