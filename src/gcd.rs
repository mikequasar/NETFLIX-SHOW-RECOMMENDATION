
//! Greatest common divisor.

use crate::{ibig::IBig, ops::DivRem, ubig::UBig};
use core::mem;

impl UBig {
    /// Greatest common divisor.
    ///
    /// # Example
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(12).gcd(&ubig!(18)), ubig!(6));
    /// ```
    ///
    /// # Panics
    ///
    /// `ubig!(0).gcd(&ubig!(0))` panics.
    pub fn gcd(&self, rhs: &UBig) -> UBig {
        let (mut a, mut b) = (self.clone(), rhs.clone());

        let zeros = match (a.trailing_zeros(), b.trailing_zeros()) {
            (None, None) => panic!("gcd(0, 0)"),
            (None, Some(_)) => return b,
            (Some(_), None) => return a,
            (Some(a_zeros), Some(b_zeros)) => {
                a >>= a_zeros;
                b >>= b_zeros;
                a_zeros.min(b_zeros)
            }
        };

        // One round of Euclidean algorithm.
        if a < b {
            mem::swap(&mut a, &mut b);
        }
        a %= &b;

        // Binary algorithm.
        loop {
            // b is odd
            match a.trailing_zeros() {
                None => break,