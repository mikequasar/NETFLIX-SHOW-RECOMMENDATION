
//! Division operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    div, helper_macros,
    ibig::IBig,
    memory::MemoryAllocation,
    ops::{Abs, DivEuclid, DivRem, DivRemEuclid, RemEuclid},
    primitive::{PrimitiveSigned, PrimitiveUnsigned},
    shift,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    convert::TryFrom,
    fmt::Debug,
    mem,
    ops::{Div, DivAssign, Rem, RemAssign},
};

impl Div<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn div(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(word0, word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0, buffer1)
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl Div<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn div(self, rhs: &UBig) -> UBig {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(word0, *word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0, buffer1.clone())
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl Div<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn div(self, rhs: UBig) -> UBig {
        match (self.repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(*word0, word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0.clone(), word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0.clone(), buffer1)
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl Div<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn div(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_word(*word0, *word1),
            (Small(_), Large(_)) => UBig::from_word(0),
            (Large(buffer0), Small(word1)) => UBig::div_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_large(buffer0.clone(), buffer1.clone())
                } else {
                    UBig::from_word(0)
                }
            }
        }
    }
}

impl DivAssign<UBig> for UBig {
    #[inline]
    fn div_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&UBig> for UBig {
    #[inline]
    fn div_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn rem(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(word0, word1),
            (Small(word0), Large(_)) => UBig::from_word(word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(&buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0, buffer1)
                } else {
                    buffer0.into()
                }
            }
        }
    }
}

impl Rem<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn rem(self, rhs: &UBig) -> UBig {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(word0, *word1),
            (Small(word0), Large(_)) => UBig::from_word(word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(&buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0, buffer1.clone())
                } else {
                    buffer0.into()
                }
            }
        }
    }
}

impl Rem<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn rem(self, rhs: UBig) -> UBig {
        match (self.repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(*word0, word1),
            (Small(word0), Large(_)) => UBig::from_word(*word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(buffer0, word1),
            (Large(buffer0), Large(mut buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0.clone(), buffer1)
                } else {
                    // Reuse buffer1 for the remainder.
                    buffer1.resizing_clone_from(buffer0);
                    buffer1.into()
                }
            }
        }
    }
}

impl Rem<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn rem(self, rhs: &UBig) -> UBig {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::rem_word(*word0, *word1),
            (Small(word0), Large(_)) => UBig::from_word(*word0),
            (Large(buffer0), Small(word1)) => UBig::rem_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::rem_large(buffer0.clone(), buffer1.clone())
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl RemAssign<UBig> for UBig {
    #[inline]
    fn rem_assign(&mut self, rhs: UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&UBig> for UBig {
    #[inline]
    fn rem_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem(self, rhs: UBig) -> (UBig, UBig) {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(word0, word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0, word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0, buffer1)
                } else {
                    (UBig::from_word(0), buffer0.into())
                }
            }
        }
    }
}

impl DivRem<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem(self, rhs: &UBig) -> (UBig, UBig) {
        match (self.into_repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(word0, *word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0, *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0, buffer1.clone())
                } else {
                    (UBig::from_word(0), buffer0.into())
                }
            }
        }
    }
}

impl DivRem<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem(self, rhs: UBig) -> (UBig, UBig) {
        match (self.repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(*word0, word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(*word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0.clone(), word1),
            (Large(buffer0), Large(mut buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0.clone(), buffer1)
                } else {
                    // Reuse buffer1 for the remainder.
                    buffer1.resizing_clone_from(buffer0);
                    (UBig::from_word(0), buffer1.into())
                }
            }
        }
    }
}

impl DivRem<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem(self, rhs: &UBig) -> (UBig, UBig) {
        match (self.repr(), rhs.repr()) {
            (Small(word0), Small(word1)) => UBig::div_rem_word(*word0, *word1),
            (Small(word0), Large(_)) => (UBig::from_word(0), UBig::from_word(*word0)),
            (Large(buffer0), Small(word1)) => UBig::div_rem_large_word(buffer0.clone(), *word1),
            (Large(buffer0), Large(buffer1)) => {
                if buffer0.len() >= buffer1.len() {
                    UBig::div_rem_large(buffer0.clone(), buffer1.clone())
                } else {
                    (UBig::from_word(0), self.clone())
                }
            }
        }
    }
}

impl DivEuclid<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: UBig) -> UBig {
        self / rhs
    }
}

impl DivEuclid<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn div_euclid(self, rhs: &UBig) -> UBig {
        self / rhs
    }
}

impl RemEuclid<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: UBig) -> UBig {
        self % rhs
    }
}

impl RemEuclid<&UBig> for &UBig {
    type Output = UBig;

    #[inline]
    fn rem_euclid(self, rhs: &UBig) -> UBig {
        self % rhs
    }
}

impl DivRemEuclid<UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl DivRemEuclid<&UBig> for &UBig {
    type OutputDiv = UBig;
    type OutputRem = UBig;

    #[inline]
    fn div_rem_euclid(self, rhs: &UBig) -> (UBig, UBig) {
        self.div_rem(rhs)
    }
}

impl Div<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn div(self, rhs: IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn div(self, rhs: &IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn div(self, rhs: IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl Div<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn div(self, rhs: &IBig) -> IBig {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 / mag1)
    }
}

impl DivAssign<IBig> for IBig {
    #[inline]
    fn div_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) / rhs;
    }
}

impl DivAssign<&IBig> for IBig {
    #[inline]
    fn div_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) / rhs;
    }
}

impl Rem<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn rem(self, rhs: IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (_, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn rem(self, rhs: &IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = self.into_sign_magnitude();
        let mag1 = rhs.magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn rem(self, rhs: IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (_, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl Rem<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn rem(self, rhs: &IBig) -> IBig {
        // Remainder with truncating division has same sign as lhs.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let mag1 = rhs.magnitude();
        IBig::from_sign_magnitude(sign0, mag0 % mag1)
    }
}

impl RemAssign<IBig> for IBig {
    #[inline]
    fn rem_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) % rhs;
    }
}

impl RemAssign<&IBig> for IBig {
    #[inline]
    fn rem_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) % rhs;
    }
}

impl DivRem<IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem(self, rhs: IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<&IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem(self, rhs: &IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem(self, rhs: IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = rhs.into_sign_magnitude();
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivRem<&IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem(self, rhs: &IBig) -> (IBig, IBig) {
        // Truncate towards 0.
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        let (q, r) = mag0.div_rem(mag1);
        (
            IBig::from_sign_magnitude(sign0 * sign1, q),
            IBig::from_sign_magnitude(sign0, r),
        )
    }
}

impl DivEuclid<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn div_euclid(self, rhs: IBig) -> IBig {
        let s = rhs.signum();
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - s,
        }
    }
}

impl DivEuclid<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn div_euclid(self, rhs: &IBig) -> IBig {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - rhs.signum(),
        }
    }
}

impl DivEuclid<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn div_euclid(self, rhs: IBig) -> IBig {
        let s = rhs.signum();
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - s,
        }
    }
}

impl DivEuclid<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn div_euclid(self, rhs: &IBig) -> IBig {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => q,
            Negative => q - rhs.signum(),
        }
    }
}

impl RemEuclid<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn rem_euclid(self, rhs: IBig) -> IBig {
        let r = self % &rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl RemEuclid<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn rem_euclid(self, rhs: &IBig) -> IBig {
        let r = self % rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl RemEuclid<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn rem_euclid(self, rhs: IBig) -> IBig {
        let r = self % &rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl RemEuclid<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn rem_euclid(self, rhs: &IBig) -> IBig {
        let r = self % rhs;
        match r.sign() {
            Positive => r,
            Negative => r + rhs.abs(),
        }
    }
}

impl DivRemEuclid<IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem_euclid(self, rhs: IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(&rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

impl DivRemEuclid<&IBig> for IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem_euclid(self, rhs: &IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

impl DivRemEuclid<IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem_euclid(self, rhs: IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(&rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

impl DivRemEuclid<&IBig> for &IBig {
    type OutputDiv = IBig;
    type OutputRem = IBig;

    #[inline]
    fn div_rem_euclid(self, rhs: &IBig) -> (IBig, IBig) {
        let (q, r) = self.div_rem(rhs);
        match r.sign() {
            Positive => (q, r),
            Negative => (q - rhs.signum(), r + rhs.abs()),
        }
    }
}

macro_rules! impl_div_ubig_unsigned {
    ($t:ty) => {
        impl Div<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn div(self, rhs: $t) -> UBig {
                self.div_unsigned(rhs)
            }
        }

        impl Div<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn div(self, rhs: $t) -> UBig {
                self.div_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Div<$t> for UBig, div);

        impl DivAssign<$t> for UBig {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.div_assign_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_assign_arg_by_value!(impl DivAssign<$t> for UBig, div_assign);

        impl Rem<$t> for UBig {
            type Output = $t;

            #[inline]
            fn rem(self, rhs: $t) -> $t {
                self.rem_unsigned(rhs)
            }
        }

        impl Rem<$t> for &UBig {
            type Output = $t;

            #[inline]
            fn rem(self, rhs: $t) -> $t {
                self.rem_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Rem<$t> for UBig, rem);

        impl RemAssign<$t> for UBig {
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                self.rem_assign_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_assign_arg_by_value!(impl RemAssign<$t> for UBig, rem_assign);

        impl DivRem<$t> for UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_unsigned(rhs)
            }
        }

        impl DivRem<$t> for &UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_div_rem_second_arg_by_value!(impl DivRem<$t> for UBig, div_rem);

         impl DivEuclid<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn div_euclid(self, rhs: $t) -> UBig {
                self.div_unsigned(rhs)
            }
        }

        impl DivEuclid<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn div_euclid(self, rhs: $t) -> UBig {
                self.div_ref_unsigned(rhs)
            }
        }
        helper_macros::forward_binop_second_arg_by_value!(impl DivEuclid<$t> for UBig, div_euclid);

        impl RemEuclid<$t> for UBig {
            type Output = $t;

            #[inline]
            fn rem_euclid(self, rhs: $t) -> $t {
                self.rem_unsigned(rhs)
            }
        }

        impl RemEuclid<$t> for &UBig {
            type Output = $t;

            #[inline]
            fn rem_euclid(self, rhs: $t) -> $t {
                self.rem_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl RemEuclid<$t> for UBig, rem_euclid);

        impl DivRemEuclid<$t> for UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem_euclid(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_unsigned(rhs)
            }
        }

        impl DivRemEuclid<$t> for &UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem_euclid(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_div_rem_second_arg_by_value!(impl DivRemEuclid<$t> for UBig, div_rem_euclid);
    };
}

impl_div_ubig_unsigned!(u8);
impl_div_ubig_unsigned!(u16);
impl_div_ubig_unsigned!(u32);
impl_div_ubig_unsigned!(u64);
impl_div_ubig_unsigned!(u128);
impl_div_ubig_unsigned!(usize);

macro_rules! impl_div_ubig_signed {
    ($t:ty) => {
        impl Div<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn div(self, rhs: $t) -> UBig {
                self.div_signed(rhs)
            }
        }

        impl Div<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn div(self, rhs: $t) -> UBig {
                self.div_ref_signed(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Div<$t> for UBig, div);

        impl DivAssign<$t> for UBig {
            #[inline]
            fn div_assign(&mut self, rhs: $t) {
                self.div_assign_signed(rhs)
            }
        }

        helper_macros::forward_binop_assign_arg_by_value!(impl DivAssign<$t> for UBig, div_assign);

        impl Rem<$t> for UBig {
            type Output = $t;

            #[inline]
            fn rem(self, rhs: $t) -> $t {
                self.rem_signed(rhs)
            }
        }

        impl Rem<$t> for &UBig {
            type Output = $t;

            #[inline]
            fn rem(self, rhs: $t) -> $t {
                self.rem_ref_signed(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Rem<$t> for UBig, rem);

        impl RemAssign<$t> for UBig {
            #[inline]
            fn rem_assign(&mut self, rhs: $t) {
                self.rem_assign_signed(rhs)
            }
        }

        helper_macros::forward_binop_assign_arg_by_value!(impl RemAssign<$t> for UBig, rem_assign);

        impl DivRem<$t> for UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_signed(rhs)
            }
        }

        impl DivRem<$t> for &UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_ref_signed(rhs)
            }
        }

        helper_macros::forward_div_rem_second_arg_by_value!(impl DivRem<$t> for UBig, div_rem);

        impl DivEuclid<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn div_euclid(self, rhs: $t) -> UBig {
                self.div_euclid_signed(rhs)
            }
        }

        impl DivEuclid<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn div_euclid(self, rhs: $t) -> UBig {
                self.div_euclid_ref_signed(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl DivEuclid<$t> for UBig, div_euclid);

        impl RemEuclid<$t> for UBig {
            type Output = $t;

            #[inline]
            fn rem_euclid(self, rhs: $t) -> $t {
                self.rem_euclid_signed(rhs)
            }
        }

        impl RemEuclid<$t> for &UBig {
            type Output = $t;

            #[inline]
            fn rem_euclid(self, rhs: $t) -> $t {
                self.rem_euclid_ref_signed(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl RemEuclid<$t> for UBig, rem_euclid);

        impl DivRemEuclid<$t> for UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem_euclid(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_euclid_signed(rhs)
            }
        }

        impl DivRemEuclid<$t> for &UBig {
            type OutputDiv = UBig;
            type OutputRem = $t;

            #[inline]
            fn div_rem_euclid(self, rhs: $t) -> (UBig, $t) {
                self.div_rem_euclid_ref_signed(rhs)
            }
        }

        helper_macros::forward_div_rem_second_arg_by_value!(impl DivRemEuclid<$t> for UBig, div_rem_euclid);
    };
}

impl_div_ubig_signed!(i8);
impl_div_ubig_signed!(i16);
impl_div_ubig_signed!(i32);
impl_div_ubig_signed!(i64);
impl_div_ubig_signed!(i128);
impl_div_ubig_signed!(isize);

macro_rules! impl_div_ibig_unsigned {
    ($t:ty) => {
        impl Rem<$t> for IBig {
            // Can be negative, so does not fit in $t.
            type Output = IBig;

            #[inline]
            fn rem(self, rhs: $t) -> IBig {
                self.rem_unsigned(rhs)
            }
        }

        impl Rem<$t> for &IBig {
            type Output = IBig;
