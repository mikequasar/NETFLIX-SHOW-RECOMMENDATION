
//! Modular addition and subtraction.

use crate::{
    add, cmp,
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::ModuloRingSmall,
    },
};
use core::{
    cmp::Ordering,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

impl<'a> Neg for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn neg(mut self) -> Modulo<'a> {
        match self.repr_mut() {
            ModuloRepr::Small(self_small) => self_small.negate_in_place(),
            ModuloRepr::Large(self_large) => self_large.negate_in_place(),
        }
        self
    }
}

impl<'a> Neg for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn neg(self) -> Modulo<'a> {
        self.clone().neg()
    }
}

impl<'a> Add<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn add(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.add(&rhs)
    }
}

impl<'a> Add<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn add(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.add_assign(rhs);
        self
    }
}

impl<'a> Add<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn add(self, rhs: Modulo<'a>) -> Modulo<'a> {
        rhs.add(self)
    }
}

impl<'a> Add<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn add(self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.clone().add(rhs)
    }
}

impl<'a> AddAssign<Modulo<'a>> for Modulo<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: Modulo<'a>) {
        self.add_assign(&rhs)
    }
}

impl<'a> AddAssign<&Modulo<'a>> for Modulo<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &Modulo<'a>) {
        match (self.repr_mut(), rhs.repr()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.add_in_place(rhs_small)
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.add_in_place(rhs_large)
            }
            _ => Modulo::panic_different_rings(),
        }
    }
}

impl<'a> Sub<Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn sub(self, rhs: Modulo<'a>) -> Modulo<'a> {
        self.sub(&rhs)
    }
}

impl<'a> Sub<&Modulo<'a>> for Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn sub(mut self, rhs: &Modulo<'a>) -> Modulo<'a> {
        self.sub_assign(rhs);
        self
    }
}

impl<'a> Sub<Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn sub(self, mut rhs: Modulo<'a>) -> Modulo<'a> {
        match (self.repr(), rhs.repr_mut()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.sub_in_place_swap(rhs_small)
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.sub_in_place_swap(rhs_large)
            }
            _ => Modulo::panic_different_rings(),
        }
        rhs
    }
}

impl<'a> Sub<&Modulo<'a>> for &Modulo<'a> {
    type Output = Modulo<'a>;

    #[inline]
    fn sub(self, rhs: &Modulo<'a>) -> Modulo<'a> {