use crate::mul::ntt::{Prime, NUM_PRIMES};

/// Maximum order of the number-theoretic transform.
///
/// 2^57 * 64 = 2^63 bits.
pub(crate) const MAX_ORDER: u32