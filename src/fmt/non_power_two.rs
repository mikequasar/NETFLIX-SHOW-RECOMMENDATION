
//! Format in a non-power-of-two radix.

use crate::{
    arch::word::Word,
    div,
    fmt::{digit_writer::DigitWriter, InRadixFull, PreparedForFormatting},
    ops::DivRem,
    radix::{self, Digit},
    ubig::{Repr::*, UBig},
};
use alloc::vec::Vec;
use core::{
    fmt::{self, Formatter},
    mem,
};
use static_assertions::const_assert;

/// Format in chunks of CHUNK_LEN * digits_per_word.
const CHUNK_LEN: usize = 16;

impl InRadixFull<'_> {
    pub(crate) fn fmt_non_power_two(&self, f: &mut Formatter) -> fmt::Result {
        debug_assert!(radix::is_radix_valid(self.radix) && !self.radix.is_power_of_two());
        match self.magnitude.repr() {
            Small(word) => {
                let mut prepared = PreparedWord::new(*word, self.radix, 1);
                self.format_prepared(f, &mut prepared)
            }
            Large(buffer) => {
                let radix_info = radix::radix_info(self.radix);
                let max_digits = buffer.len() * (radix_info.digits_per_word + 1);