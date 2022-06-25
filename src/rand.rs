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
        debug_assert!(*range !=