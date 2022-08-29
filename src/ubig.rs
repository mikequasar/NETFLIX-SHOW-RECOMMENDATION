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
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) enum Repr {
    /// A number that fits in a single Word.
    Small(Word),
    /// A number that does not fit in a single Word.
    ///
    /// The buffer has:
    /// *