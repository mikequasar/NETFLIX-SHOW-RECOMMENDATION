use crate::arch::word::Word;

/// Add a + b + carry.
///
/// Returns (result, overflow).
#[inline]
pub(crate) fn add_with_carry(a: Word, b: Word, carry: bool) -> (Word, bool) {
    let (sum, c0) = a.overflowing_add(b);
    let (sum, c1)