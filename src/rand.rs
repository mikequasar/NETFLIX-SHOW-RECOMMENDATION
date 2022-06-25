//! Random distributions.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    ibig::IBig,
    ops::UnsignedAbs,
    ubig::{Repr::*, UBig},
};

use rand::{
    distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler},
    Rng,
};

impl SampleUniform for UBig {
    type Sampler = UniformUBig;
}

impl SampleUniform for IBig {
    type Sampler = UniformIBig;
}

impl UBig {
    /// Random UBig in range [0..range)
    #[inline]
    fn uniform<R>(range: &UBig, rng: &mut R) -> UBig
    where
        R: Rng + ?Sized,
    {
        debug_assert!(*range != UBig::from_word(0));

        match range.repr() {
            Small(word) => UBig::from_word(rng.gen_range(0..*word)),
            Large(buffer) => UBig::uniform_large(buffer, rng),
        }
    }

    /// Random UBig in ra