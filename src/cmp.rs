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
            (Small(word), Small(other_w