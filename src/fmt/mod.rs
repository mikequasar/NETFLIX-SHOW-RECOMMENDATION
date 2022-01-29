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
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: DigitCase::Lower,
        }
        .fmt(f)
    }
}

impl UpperHex for UBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: Positive,
            magnitude: self,
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: DigitCase::Upper,
        }
        .fmt(f)
    }
}

impl Display for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 10,
            prefix: "",
            digit_case: DigitCase::NoLetters,
        }
        .fmt(f)
    }
}

impl Debug for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Binary for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 2,
            prefix: if f.alternate() { "0b" } else { "" },
            digit_case: DigitCase::NoLetters,
        }
        .fmt(f)
    }
}

impl Octal for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 8,
            prefix: if f.alternate() { "0o" } else { "" },
            digit_case: DigitCase::NoLetters,
        }
        .fmt(f)
    }
}

impl LowerHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: DigitCase::Lower,
        }
        .fmt(f)
    }
}

impl UpperHex for IBig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        InRadixFull {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix: 16,
            prefix: if f.alternate() { "0x" } else { "" },
            digit_case: DigitCase::Upper,
        }
        .fmt(f)
    }
}

impl UBig {
    /// Representation in a given radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(format!("{}", ubig!(83).in_radix(3)), "10002");
    /// assert_eq!(format!("{:+010}", ubig!(35).in_radix(36)), "+00000000z");
    /// ```
    #[inline]
    pub fn in_radix(&self, radix: u32) -> InRadix {
        radix::check_radix_valid(radix);
        InRadix {
            sign: Positive,
            magnitude: self,
            radix,
        }
    }
}

impl IBig {
    /// Representation in a given radix.
    ///
    /// # Panics
    ///
    /// Panics if `radix` is not between 2 and 36 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ibig;
    /// assert_eq!(format!("{}", ibig!(-83).in_radix(3)), "-10002");
    /// assert_eq!(format!("{:010}", ibig!(-35).in_radix(36)), "-00000000z");
    /// ```
    #[inline]
    pub fn in_radix(&self, radix: u32) -> InRadix {
        radix::check_radix_valid(radix);
        InRadix {
            sign: self.sign(),
            magnitude: self.magnitude(),
            radix,
        }
    }
}

/// Representation of a [UBig] or [IBig] in any radix between 2 and 36 inclusive.
///
/// This can be used to format a number in a non-standard radix.
///
/// The default format uses lower-case letters a-z for digits 10-35.
/// The "alternative" format (`{:#}`) uses upper-case letters.
///
/// # Examples
///
/// ```
/// # use ibig::{ibig, ubig};
/// assert_eq!(format!("{}", ubig!(83).in_radix(3)), "10002");
/// assert_eq!(format!("{:+010}", ubig!(35).in_radix(36)), "+00000000z");
/// // For bases 2, 8, 10, 16 we don't have to use `InRadix`:
/// assert_eq!(format!("{:x}", ubig!(3000)), "bb8");
/// assert_eq!(format!("{:x}", ibig!(-3000)), "-bb8");
/// ```
pub struct InRadix<'a> {
    sign: Sign,
    magnitude: &'a UBig,
    radix: Digit,
}

/// Representation in a given radix with a prefix and digit case.
struct InRadixFull<'a> {
    sign: Sign,
    magnitude: &'a UBig,
    radix: Digit,
    prefix: &'static str,
    digit_case: DigitCase,
}

impl Display for InRadix<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let digit_case = if self.radix <= 10 {
            DigitCase::NoLetters
        } else if f.alternate() {
            DigitCase::Upper
        } else {
            DigitCase::Lower
        };

        InRadixFull {
            sign: self.sign,
            magnitude: self.magnitude,
            radix: self.radix,
            prefix: "",
            d