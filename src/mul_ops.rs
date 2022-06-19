
//! Multiplication operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    helper_macros,
    ibig::IBig,
    memory::MemoryAllocation,
    mul,
    primitive::{extend_word, PrimitiveSigned, PrimitiveUnsigned},
    sign::Sign::{self, *},
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Mul, MulAssign},
};
use static_assertions::const_assert;

impl Mul<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn mul(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(word0, word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1, word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(&buffer0, &buffer1),
        }
    }
}

impl Mul<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn mul(self, rhs: &UBig) -> UBig {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1.clone(), word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(&buffer0, buffer1),
        }
    }
}

impl Mul<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn mul(self, rhs: UBig) -> UBig {
        rhs.mul(self)
    }
}

impl Mul<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn mul(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::mul_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::mul_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => UBig::mul_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => UBig::mul_large(buffer0, buffer1),
        }
    }
}

impl MulAssign<UBig> for UBig {
    #[inline]
    fn mul_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&UBig> for UBig {
    #[inline]