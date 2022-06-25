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

impl SampleUniform for IBig 