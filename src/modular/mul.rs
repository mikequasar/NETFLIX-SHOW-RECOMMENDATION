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
    #[inline]
    fn mul_assign(&mut self, rhs: &Modulo<'a>) {
        match (self.repr_mut(), rhs.repr()) {
            (ModuloRepr::Small(self_small), ModuloRepr::Small(rhs_small)) => {
                self_small.check_same_ring(rhs_small);
                self_small.mul_in_place(rhs_small);
            }
            (ModuloRepr::Large(self_large), ModuloRepr::Large(rhs_large)) => {
                self_large.check_same_ring(rhs_large);
                let memory_requirement = self_large.ring().mul_memory_requirement();
                let mut allocation = MemoryAllocation::new(memory_requirement);
                let mut memory = allocation.memory();
                self_large.mul_in_place(rhs_large, &mut memory);
            }
            _ => Modulo::panic_different_rings(),
        }
    }
}

impl ModuloSmallRaw {
    #[inline]
    pub(crate) const fn mul(self, other: ModuloSmallRaw, ring: &ModuloRingSmall) -> ModuloSmallRaw {
        debug_assert_in_const_fn!(self.is_valid(ring) && other.is_valid(ring));
        let a = self.normalized();
        let b = other.normalized();
        let product = extend_word(a >> ring.shift()) * extend_word(b);
        let (_, rem) = ring.fast_div().div_rem(product);
        ModuloSmallRaw::from_normalized(rem)
    }
}

impl<'a> ModuloSmall<'a> {
    /// self *= rhs
    #[inline]
    pub(crate) fn mul_in_place(&mut self, rhs: &ModuloSmall<'a>) {
        self.check_same_ring(rhs);
        self.set_raw(self.raw().mul(rhs.raw(), self.ring()));
    }
}

impl ModuloRingLarge {
    pub(crate) fn mul_memory_requirement(&self) -> Layout {
        let n = self.normalized_modulus().len();
        memory::add_layout(
            memory::array_layout::<Word>(2 * n),
            memory::max_layout(
                mul::memory_requirement_exact(2 * n, n),
                div::memory_requirement_exact(2 * n, n),
            ),
        )
    }

    /// Returns a * b allocated in memory.
    pub(crate) fn mul_normalized<'a>(
        &self,
        a: &[Word],
        b: &[Word],
        memory: &'a mut Memory,
    ) -> &'a [Word] {
        let modulus = self.normalized_modulus();
        let n = modulus.len();
        debug_assert!(a.len() == n && b.len() == n);

        let (product, mu