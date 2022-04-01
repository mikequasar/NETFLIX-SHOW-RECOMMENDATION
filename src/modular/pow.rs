use crate::{
    arch::word::Word,
    ibig::IBig,
    math,
    memory::{self, MemoryAllocation},
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::ModuloRingSmall,
    },
    primitive::{double_word, split_double_word, PrimitiveUnsigned, WORD_BITS, WORD_BITS_USIZE},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};

impl<'a> Modulo<'a> {
    /// Exponentiation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ubig};
    /// // A Mersenne prime.
    /// let p = ubig!(2).pow(607) - ubig!(1);
    /// let ring = ModuloRing::new(&p);
    /// // Fermat's little theorem: a^(p-1) = 1 (mod p)
    /// let a = ring.from(123);
    /// assert_eq!(a.pow(&(p - ubig!(1))), ring.from(1));
    /// ```
    #[inline]
    pub fn pow(&self, exp: &UBig) -> Modulo<'a> {
        match self.repr() {
            ModuloRepr::Small(self_small) => self_small.pow(exp).into(),
            ModuloRepr::Large(self_large) => self_large.pow(exp).into(),
        }
    }

    /// Exponentiation to a signed exponent.
    ///
    /// # Panic
    ///
    /// Panics if the exponent is negative and the base is not invertible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{modular::ModuloRing, ibig, ubig};
    /// let ring = ModuloRing::new(&ubig!(10));
    /// assert_eq!(ring.from(2).pow_signed(&ibig!(4)), ring.from(6));
    /// assert_eq!(ring.from(3).pow_signed(&ibig!(-3)), ring.from(3));
    /// ```
    #[inline]
    pub fn pow_signed(&self, exp: &IBig) -> Modulo<'a> {
        match exp.sign() {
            Positive => self.pow(exp.magnitude()),
            Negative => match self.inverse() {
                None => panic!("Non-invertible Modulo taken to a negative power"),
                Some(inv) => inv.pow(exp.magnitude()),
            },
        }
    }
}

impl ModuloSmallRaw {
    /// self^exp
    #[inline]
    pub(crate) const fn pow_word(self, exp: Word, ring: &ModuloRingSmall) -> ModuloSmallRaw {
        if exp == 0 {
            return ModuloSmallRaw::from_word(1, ring);
        }

        let bits = WORD_BITS - 1 - exp.leading_zeros();
        self.pow_helper(bits, self, exp, ring)
    }

    /// self^2^bits * base^exp[..bits]
    #[inline]
    const fn pow_helper(
        self,
        mut bits: u32,
        base: ModuloSmallRaw,
        exp: Word,
        ring: &ModuloRingSmall,
    ) -> ModuloSmallRaw {
        let mut res = self;
        while bits > 0 {
            res = res.mul(res, ring);
            bits -= 1;
            if exp & (1 << bits) != 0 {
                res = res.mul(base, ring);
            }
        }
        res
    }
}

impl<'a> ModuloSmall<'a> {
    /// Exponentiation.
    #[inline]
    fn pow(&self, exp: &UBig) -> ModuloSmall<'a> {
        match exp.repr() {
            // self^0 == 1
            Small(0) => ModuloSmall::from_ubig(&UBig::from_word(1), self.ring()),
            // self^1 == self
            Small(1) => self.clone(),
            // self^2 == self * self
            Small(2) => {
                let res = self.raw().mul(self.raw(), self.ring());
                ModuloSmall::new(res, self.ring())
            }
            _