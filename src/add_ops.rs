//! Addition and subtraction operators.

use crate::{
    add,
    arch::word::Word,
    buffer::Buffer,
    helper_macros,
    ibig::IBig,
    primitive::{PrimitiveSigned, PrimitiveUnsigned},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Add, AddAssign, Sub, SubAssign},
};

impl Add<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn add(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {