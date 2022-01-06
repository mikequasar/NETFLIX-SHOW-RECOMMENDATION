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
    fn fmt(&self, f: &mut Formatter) -> fm