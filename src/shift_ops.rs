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
           