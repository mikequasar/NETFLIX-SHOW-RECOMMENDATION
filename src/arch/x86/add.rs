use crate::arch::word::Word;

/// Add a + b + carry.
///
/// Returns (result, overflow).
#[inline]
pub(crate) fn add_with_carry(a: Word, b: Word, carry: bool) -> (Word, bool) {
    let mut sum = 0;
    let carry = unsafe { core::arch::x86::_addcarry_u32(carry.into(), a, b, &mut sum) };
    (sum, carry != 0)
}

/// 