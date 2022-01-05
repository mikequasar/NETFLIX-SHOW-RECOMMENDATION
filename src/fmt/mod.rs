//! Formatting helpers.

use crate::{
    ibig::IBig,
    radix::{self, Digit, DigitCase},
    sign::Sign::{self, *},
    ubig::UBig,
};
use core::fmt::{
    self, Alignment, Binary, Debug, Displa