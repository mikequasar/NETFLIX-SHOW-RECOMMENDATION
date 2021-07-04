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
                .le