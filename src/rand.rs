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

    /// Random UBig in range [0..words)
    fn uniform_large<R>(words: &[Word], rng: &mut R) -> UBig
    where
        R: Rng + ?Sized,
    {
        let mut buffer = Buffer::allocate(words.len());
        buffer.push_zeros(words.len());
        while !try_fill_uniform(words, rng, &mut buffer) {
            // Repeat.
        }
        buffer.into()
    }
}

/// Try to fill `sample` with random number in range [0..words).
/// May fail randomly.
///
/// Returns true on success.
fn try_fill_uniform<R>(words: &[Word], rng: &mut R, result: &mut [Word]) -> bool
where
    R: Rng + ?Sized,
{
    let n = words.len();
    debug_assert!(n > 0 && result.len() == n);
    let mut i = n - 1;
    result[i] = rng.gen_range(0..=words[i]);
    // With at least 50% probability this loop executes 0 times (and thus doesn't fail).
    while result[i] == words[i] {
        if i == 0 {
            // result == words
            return false;
        }
        i -= 1;
        result[i] = rng.gen();
        if result[i] > words[i] {
            return false;
        }
    }
    rng.fill(&mut result[..i]);
    true
}

/// Uniform [UBig] distribution.
///
/// # Example
///
/// ```
/// use ibig::ubig;
/// use rand::{distributions::uniform::Uniform, thread_rng, Rng};
/// let a = thread_rng().gen_range(ubig!(3)..ubig!(10));
/// let b = thread_rng().sample(Uniform::new(ubig!(0), &a));
/// assert!(a >= ubig!(3) && a < ubig!(10));
/// assert!(b >= ubig!(0) && b < a);
/// ```
pub struct UniformUBig {
    range: UBig,
    offset: UBig,
}

impl UniformSampler for UniformUBig {
    type X = UBig;

    #[inline]
    fn new<B1, B2>(low: B1, high: B2) -> UniformUBig
    where
        B1: SampleBorrow<UBig>,
        B2: SampleBorrow<UBig>,
    {
        let range = high.borrow() - low.borrow();
        if range == UBig::from_word(0) {
            panic!("Empty range");
        }
        UniformUBig {
            range,
            of