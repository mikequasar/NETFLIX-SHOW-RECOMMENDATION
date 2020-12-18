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
                if buffer0.len() >= buffer1.len() {
                    UBig::add_large(buffer0, &buffer1)
                } else {
                    UBig::add_large(buffer1, &buffer0)
                }
            }
        }
    }
}

impl Add<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn add(self, rhs: &UBig) -> UBig {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1.clone(), word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => UBig::add_large(buffer0, buffer1),
        }
    }
}

impl Add<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn add(self, rhs: UBig) -> UBig {
        rhs.add(self)
    }
}

impl Add<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn add(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::add_word(*word0, *word1),
            (Small(word0), Large(buffer1)) => UBig::add_large_word(buffer1.clone(), *word0),
            (Large(buffer0), Small(word1)) => UBig::add_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::add_large(buffer0.clone(), buffer1)
                } else {
                    UBig::add_large(buffer1.clone(), buffer0)
                }
            }
        }
    }
}

impl AddAssign<UBig> for UBig {
    #[inline]
    fn add_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&UBig> for UBig {
    #[inline]
    fn add_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) + rhs;
    }
}

impl Sub<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn sub(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::sub_word(word0, word1),
            (Small(_), Large(_)) => UBig::panic_negative(),
            (Large(buffer0), Small(word1)) => UBig::sub_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => UBig::sub_large(buffer0, &buffer1),
        }
    }
}

impl Sub<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn sub(self, rhs: &UBig) -> UBig {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::sub_word(word0, *word1),
            (Small(_), Large(_)) => UBig::panic_negative(),
            (Large(buffer0), Small(word1)) => UBig::sub_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => UBig::sub_large(buffer0, buffer1),
        }
    }
}

impl Sub<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn sub(self, rhs: UBig) -> UBig {
        match (self.repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::sub_word(*word0, word1),
            (Small(_), Large(_)) => UBig::panic_negative(),
            (Large(buffer0), Small(word1)) => UBig::sub_large_word(buffer0.clone(), word1),
            (Large(buffer0), Large(buffer1)) => UBig::sub_large_ref_val(buffer0, buffer1),
        }
    }
}

impl Sub<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn sub(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::sub_word(*word0, *word1),
            (Small(_), Large(_)) => UBig::panic_negative(),
            (Large(buffer0), Small(word1)) => UBig::sub_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => UBig::sub_large(buffer0.clone(), buffer1),
        }
    }
}

impl SubAssign<UBig> for UBig {
    #[inline]
    fn sub_assign(&mut self, r