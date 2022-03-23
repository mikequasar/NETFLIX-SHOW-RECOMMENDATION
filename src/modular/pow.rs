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
    /// let ring = ModuloRing