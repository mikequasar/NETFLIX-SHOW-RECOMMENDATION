//! Conversions between types.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    error::OutOfBoundsError,
    ibig::IBig,
    primitive::{self, PrimitiveSigned, PrimitiveUnsigned, WORD_BITS, WORD_BYTES},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};

impl Default for UBig {
    /// Default value: 0.
    #[inline]
    fn default() -> UBig {
        UBig::from_word(0)
    }
}