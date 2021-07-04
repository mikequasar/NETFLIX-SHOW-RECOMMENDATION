//! Comparisons operators.

use crate::{
    arch::word::Word,
    ibig::IBig,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::cmp::Ordering;

impl Ord for UBig {
    #[inline]
    fn cmp(&self, other: &UBig) -> Ordering {
        match (self.repr(), other.repr()) {
            (Small(word), Small(other_word)) => word.cmp(other_word),
            (Small(_), Large(_)) => Ordering::Less,
            (Large(_), Small(_)) => Ordering::Greater,
            (Large(buffer), Large(other_buffer)) => buffer
                .len()
                .cmp(&other_buffer.len())
                .then_with(|| cmp_same_len(buffer, other_buffer)),
        }
    }
}

impl PartialOrd for UBig {
    #[inline]
    fn partial_cmp(&self, other: &UBig) -> Op