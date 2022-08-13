//! Bit shift operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    ibig::IBig,
    primitive::{double_word, extend_word, split_double_word, WORD_BITS_USIZE},
    shift,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Shl, ShlAssign, Shr, ShrAssign},
};

macro_rules! impl_shifts {
    ($t:ty) => {
        impl Shl<&usize> for $t {
            type Output = $t;

            #[inline]
            fn shl(self, rhs: &usize) -> $t {
                self.shl(*rhs)
            }
        }

        impl Shl<&usize> for &$t {
            type Output = $t;

            #[inline]
            fn shl(self, rhs: &usize) -> $t {
                self.shl(*rhs)
            }
        }

        impl ShlAssign<usize> for $t {
            #[inline]
            fn shl_assign(&mut self, rhs: usize) {
                *self = mem::take(self) << rhs;
            }
        }

        impl ShlAssign<&usize> for $t {
            #[inline]
            fn shl_assign(&mut self, rhs: &usize) {
                *self = mem::take(self) << rhs;
            }
        }

        impl Shr<&usize> for $t {
            type Output = $t;

            #[inline]
            fn shr(self, rhs: &usize) -> $t {
                self.shr(*rhs)
            }
        }

        impl Shr<&usize> for &$t {
            type Output = $t;

            #[inline]
            fn shr(self, rhs: &usize) -> $t {
                self.shr(*rhs)
            }
        }

        impl ShrAssign<usize> for $t {
            #[inline]
            fn shr_assign(&mut self, rhs: usize) {
                *self = mem::take(self).shr(rhs);
            }
        }

        impl ShrAssign<&usize> for $t {
            #[inline]
            fn shr_assign(&mut self, rhs: &usize) {
                *self = mem::take(self).shr(rhs);
            }
        }
    };
}

impl_shifts!(UBig);
impl_shifts!(IBig);

impl Shl<usize> for UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: usize) -> UBig {
        match self.into_repr() {
            Small(0) => UBig::from_word(0),
            Small(word) => UBig::shl_word(word, rhs),
            Large(buffer) => UBig::shl_large(buffer, rhs),
        }
    }
}

impl Shl<usize> for &UBig {
    type Output = UBig;

    #[inline]
    fn shl(self, rhs: usize) -> UBig {
        match self.repr() {
            Small(0) => UBig::from_word(0),
            Small(word) => UBig::shl_word(*word, rhs),
            Large(buffer) => UBig::shl_ref_large(buffer, rhs),
        }
    }
}

impl Shr<usize> for UBig {
    type Output = UBig;

    #[inline]
    fn shr(self, rhs: usize) -> UBig {
        match self.into_repr() {
            Small(word) => UBig::shr_word(word, rhs),
            Large(buffer) => UBig::shr_large(buffer, rhs),
        }
    }
}

impl Shr<usize> for &UBig {
    type Output = UBig;

    #[inline]
    fn shr(self, rhs: usize) -> UBig {
        match self.repr() {
            Small(word) => UBig::shr_word(*word, rhs),
            Large(buffer) => UBig::shr_large_ref(buffer, rhs),
        }
    }
}

impl Shl<usize> for IBig {
    type Output = IBig;

    #[inline]
    fn shl(self, rhs: usize) -> IBig {
        let (sign, mag) = self.into_sign_magnitude();
        IBig::from_sign_magnitude(sign, mag.shl(rhs))
    }
}

impl Shl<usize> for &IBig {
    type Output = IBig;

    #[inline]
    fn shl(self, rhs: usize) -> IBig {
        let (sign, mag) = (self.sign(), self.magnitude());
        IBig::from_sign_magnitude(sign, mag.shl(rhs))
    }
}

impl Shr<usize> for IBig {
    type Output = IBig;

    #[inline]
    fn shr(self, rhs: usize) -> IBig {
        let (sign, mag) = self.into_sign_magnitude();
        match sign {
            Positive => IBig::from(mag.shr(rhs)),
            Negative => {
                let b = mag.are_low_bits_nonzero(rhs);
                -IBig::from(mag.shr(rhs)) - IBig::from(b)
            }
        }
    }
}

impl Shr<usize> for &IBig {
    type Output = IBig;

    #[inline]
    fn shr(self, rhs: usize) -> IBig {
        let (sign, mag) = (self.sign(), self.magnitude());
        match sign {
            Positive => IBig::from(mag.shr(rhs)),
            Negative => {
                let b = mag.are_low_bits_nonzero(rhs);
                -IBig::from(mag.shr(rhs)) - IBig::from(b)
            }
        }
    }
}

impl UBig {
    /// Shift left one non-zero `Word` by `rhs` bits.
    #[inline]
    fn shl_word(word: Word, rhs: usize) -> UBig {
        debug_assert!(word != 0);

        if rhs <= WORD_BITS_USIZE {
            UBig::from(extend_word(word) << rhs)
        } else {
            UBig::shl_word_slow(word, rhs)
        }
    }

    /// Shift left one non-zero `Word` by `rhs` bits.
    fn shl_word_slow(word: Word, rhs: usize) -> UBig {
        let shift_words = rhs / WORD_BITS_USIZE;
        let shift_bits = (rhs % WORD_BITS_USIZE) as u32;
        let (lo, hi) = split_double_word(extend_word(word) << shift_bits);
 