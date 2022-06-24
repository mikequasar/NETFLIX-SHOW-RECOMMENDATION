
//! Information about radixes.

use crate::{
    arch::word::Word,
    fast_divide::{FastDivideNormalized, FastDivideSmall},
    primitive::WORD_BITS,
};
use static_assertions::const_assert;

/// Digit and radix type.
pub(crate) type Digit = u32;

/// Maximum supported radix.
pub(crate) const MAX_RADIX: Digit = 36;

/// Is a radix in valid range?
#[inline]
pub(crate) fn is_radix_valid(radix: Digit) -> bool {
    (2..=MAX_RADIX).contains(&radix)
}

/// Panics if `radix` is not in valid range.
#[inline]
pub(crate) fn check_radix_valid(radix: Digit) {
    if !is_radix_valid(radix) {
        panic!("Invalid radix: {}", radix);
    }
}

const_assert!(b'a' > b'0' + 10 && b'A' > b'0' + 10);

/// u8 representation is: how much digits >= 10 should be offset by in ASCII.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum DigitCase {
    NoLetters = 0,
    Lower = b'a' - b'0' - 10,
    Upper = b'A' - b'0' - 10,
}

/// Converts a byte (ASCII) representation of a digit to its value.
pub(crate) fn digit_from_utf8_byte(byte: u8, radix: Digit) -> Option<Digit> {
    let res = match byte {
        b'0'..=b'9' => (byte - b'0') as Digit,
        b'a'..=b'z' => (byte - b'a') as Digit + 10,
        b'A'..=b'Z' => (byte - b'A') as Digit + 10,
        _ => return None,
    };
    if res < radix {
        Some(res)
    } else {
        None
    }
}