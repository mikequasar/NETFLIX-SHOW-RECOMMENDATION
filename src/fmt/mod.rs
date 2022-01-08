//! Formatting helpers.

use crate::{
    ibig::IBig,
    radix::{self, Digit, DigitCase},
    sign::Sign::{self, *},
    ubig::UBig,
};
use core::fmt::{
    self, Alignment, Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex, Write,
};
use digit_writer::DigitWriter;

mod digit_writer;
mod non_power_two;
mod power_two;

impl Display for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: Positive,
            magnitude: self,
            radix: 10,
            prefix: "",
            digit_case: DigitCase::NoLetters,
        }
        .fmt(f)
    }
}

impl Debug for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: Positive,
            magnitude: self,
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: DigitCase::NoLetters,
        }
        .fmt(f)
    }
}

impl Octal for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: Positive,
            magnitude: self,
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: DigitCase::NoLetters,
        }
        .fmt(f)
    }
}

impl LowerHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
