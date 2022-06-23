
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
    fn mul_assign(&mut self, rhs: &UBig) {
        *self = mem::take(self) * rhs;
    }
}

impl Mul<IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<&IBig> for IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = self.into_sign_magnitude();
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl Mul<IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: IBig) -> IBig {
        rhs.mul(self)
    }
}

impl Mul<&IBig> for &IBig {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: &IBig) -> IBig {
        let (sign0, mag0) = (self.sign(), self.magnitude());
        let (sign1, mag1) = (rhs.sign(), rhs.magnitude());
        IBig::from_sign_magnitude(sign0 * sign1, mag0 * mag1)
    }
}

impl MulAssign<IBig> for IBig {
    #[inline]
    fn mul_assign(&mut self, rhs: IBig) {
        *self = mem::take(self) * rhs;
    }
}

impl MulAssign<&IBig> for IBig {
    #[inline]
    fn mul_assign(&mut self, rhs: &IBig) {
        *self = mem::take(self) * rhs;
    }
}

impl Mul<Sign> for Sign {
    type Output = Sign;

    #[inline]
    fn mul(self, rhs: Sign) -> Sign {
        match (self, rhs) {
            (Positive, Positive) => Positive,
            (Positive, Negative) => Negative,
            (Negative, Positive) => Negative,
            (Negative, Negative) => Positive,
        }
    }
}

impl MulAssign<Sign> for Sign {
    #[inline]
    fn mul_assign(&mut self, rhs: Sign) {
        *self = *self * rhs;
    }
}

impl Mul<IBig> for Sign {
    type Output = IBig;

    #[inline]
    fn mul(self, rhs: IBig) -> IBig {
        let (sign, mag) = rhs.into_sign_magnitude();
        IBig::from_sign_magnitude(self * sign, mag)
    }
}

macro_rules! impl_mul_ubig_unsigned {
    ($t:ty) => {
        impl Mul<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn mul(self, rhs: $t) -> UBig {
                self.mul_unsigned(rhs)
            }
        }

        impl Mul<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn mul(self, rhs: $t) -> UBig {
                self.mul_ref_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Mul<$t> for UBig, mul);
        helper_macros::forward_binop_swap_args!(impl Mul<UBig> for $t, mul);

        impl MulAssign<$t> for UBig {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.mul_assign_unsigned(rhs)
            }
        }

        helper_macros::forward_binop_assign_arg_by_value!(impl MulAssign<$t> for UBig, mul_assign);
    };
}

impl_mul_ubig_unsigned!(u8);
impl_mul_ubig_unsigned!(u16);
impl_mul_ubig_unsigned!(u32);
impl_mul_ubig_unsigned!(u64);
impl_mul_ubig_unsigned!(u128);
impl_mul_ubig_unsigned!(usize);

macro_rules! impl_mul_ubig_signed {
    ($t:ty) => {
        impl Mul<$t> for UBig {
            type Output = UBig;

            #[inline]
            fn mul(self, rhs: $t) -> UBig {
                self.mul_signed(rhs)
            }
        }

        impl Mul<$t> for &UBig {
            type Output = UBig;

            #[inline]
            fn mul(self, rhs: $t) -> UBig {
                self.mul_ref_signed(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Mul<$t> for UBig, mul);
        helper_macros::forward_binop_swap_args!(impl Mul<UBig> for $t, mul);

        impl MulAssign<$t> for UBig {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.mul_assign_signed(rhs)
            }
        }

        helper_macros::forward_binop_assign_arg_by_value!(impl MulAssign<$t> for UBig, mul_assign);
    };
}

impl_mul_ubig_signed!(i8);
impl_mul_ubig_signed!(i16);
impl_mul_ubig_signed!(i32);
impl_mul_ubig_signed!(i64);
impl_mul_ubig_signed!(i128);
impl_mul_ubig_signed!(isize);

macro_rules! impl_mul_ibig_primitive {
    ($t:ty) => {
        impl Mul<$t> for IBig {
            type Output = IBig;

            #[inline]
            fn mul(self, rhs: $t) -> IBig {
                self.mul_primitive(rhs)
            }
        }

        impl Mul<$t> for &IBig {
            type Output = IBig;

            #[inline]
            fn mul(self, rhs: $t) -> IBig {
                self.mul_ref_primitive(rhs)
            }
        }

        helper_macros::forward_binop_second_arg_by_value!(impl Mul<$t> for IBig, mul);
        helper_macros::forward_binop_swap_args!(impl Mul<IBig> for $t, mul);

        impl MulAssign<$t> for IBig {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                self.mul_assign_primitive(rhs)
            }
        }
