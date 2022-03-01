use crate::{
    arch::word::Word,
    assert::debug_assert_in_const_fn,
    div,
    memory::{self, Memory, MemoryAllocation},
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::{ModuloRingLarge, ModuloRingSmall},
    },
    mul,
    primitive::extend_word,
    shift,
    sign::Sign::Positive,
};
use alloc::alloc::Layout;
use core::ops::{Mul, MulAssign};

impl<'a> Mul<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn mul(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.mul(&rhs)
    }
}

impl<'a> Mul<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn mul(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.mul_assign(rhs);
        self
    }
}

impl<'a> Mul<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn mul(self, rhs: Modulo<'a>) -> Modulo<'a> {
        rhs.mul(self)
    }
}

impl<'a> Mul<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn mul(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.clone().mul(rhs)
    }
}

impl<'a> MulAssign<Modulo<'a>> for Modulo<'a> {
    #[inline]
    fn mul_assign(&mut self, rhs: Modulo<'a>) {
        self.mul_assign(&rhs)
    }
}

impl<'a> MulAssign<&Modulo<'a>> for Modulo<'a> {
    #[inlin