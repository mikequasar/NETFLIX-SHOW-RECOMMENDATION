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
    fn sub_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) - rhs;
    }
}

impl SubAssign<&UBig> for UBig {
    #[inline]
    fn sub_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) - rhs;
    }
}

impl Add<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn add(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_val_val(mag0, mag1),
            (Negative, Positive) => IBig::sub_ubig_val_val(mag1, mag0),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl Add<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn add(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_val_ref(mag0, mag1),
            (Negative, Positive) => -IBig::sub_ubig_val_ref(mag0, mag1),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl Add<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn add(self, rhs: IBig) -> IBig {
        rhs.add(self)
    }
}

impl Add<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn add(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::from(mag0 + mag1),
            (Positive, Negative) => IBig::sub_ubig_ref_ref(mag0, mag1),
            (Negative, Positive) => IBig::sub_ubig_ref_ref(mag1, mag0),
            (Negative, Negative) => -IBig::from(mag0 + mag1),
        }
    }
}

impl AddAssign<IBig> for IBig {
    #[inline]
    fn add_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) + rhs;
    }
}

impl AddAssign<&IBig> for IBig {
    #[inline]
    fn add_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) + rhs;
    }
}

impl Sub<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn sub(self, rhs: IBig) -> IBig {
        self + -rhs
    }
}

impl Sub<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn sub(self, rhs: &IBig) -> IBig {
        -(-self + rhs)
    }
}

impl Sub<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn sub(self, rhs: IBig) -> IBig {
        self + -rhs
    }
}

impl Sub<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn sub(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        match (sign0, sign1) {
            (Positive, Positive) => IBig::sub_ubig_ref_ref(mag0, mag1),
            (Positive, Negative) => IBig::from(mag0 + mag1),
            (Negative, Positive) => -IBig::from(mag0 + mag1),
            (Negative, Negative) => IBig::sub_ubig_ref_ref(mag1, mag0),
        }
    }
}

impl SubAssign<IBig> for IBig {
    #[inline]
    fn sub_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) - rhs;
    }
}

impl SubAssign<&IBig> for IBig {
    #[inline]
    fn sub_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) - rhs;
    }
}

macro_rules! impl_add_ubig_unsigned {
    ($t:ty) => {
        impl Add<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn add(self, rhs: $t) -> UBig {
                self.add_unsigned(rhs)
            }
        }

        impl Add<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn add(self, rhs: $t) -> UBig {
                self.add_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Add<$t> for UBig, add);
        helper_macros::forward_binop_swap_args!(impl Add<UBig> for $t, add);

        impl AddAssign<$t> for UBig {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                self.add_assign_unsigned(rhs)
    