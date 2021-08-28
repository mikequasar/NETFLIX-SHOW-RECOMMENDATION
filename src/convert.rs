//! Conversions between types.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    error::OutOfBoundsError,
    ibig::IBig,
    primitive::{self, PrimitiveSigned, PrimitiveUnsigned, WORD_BITS, WORD_BYTES},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};

impl Default for UBig {
    /// Default value: 0.
    #[inline]
    fn default() -> UBig {
        UBig::from_word(0)
    }
}

impl Default for IBig {
    /// Default value: 0.
    #[inline]
    fn default() -> IBig {
        IBig::from(0u8)
    }
}

impl UBig {
    /// Construct from little-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ubig, UBig};
    /// assert_eq!(UBig::from_le_bytes(&[3, 2, 1]), ubig!(0x010203));
    /// ```
    #[inline]
    pub fn from_le_bytes(bytes: &[u8]) -> UBig {
        if bytes.len() <= WORD_BYTES {
            // fast path
            UBig::from_word(primitive::word_from_le_bytes_partial(bytes))
        } else {
            UBig::from_le_bytes_large(bytes)
        }
    }

    fn from_le_bytes_large(bytes: &[u8]) -> UBig {
        debug_assert!(bytes.len() > WORD_BYTES);
        let mut buffer = Buffer::allocate((bytes.len() - 1) / WORD_BYTES + 1);
        let mut chunks = bytes.chunks_exact(WORD_BYTES);
        for chunk in &mut chunks {
            buffer.push(Word::from_le_bytes(chunk.try_into().unwrap()));
        }
        if !chunks.remainder().is_empty() {
            buffer.push(primitive::word_from_le_bytes_partial(chunks.remainder()));
        }
        buffer.into()
    }

    /// Construct from big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::{ubig, UBig};
    /// assert_eq!(UBig::from_be_bytes(&[1, 2, 3]), ubig!(0x010203));
    /// ```
    #[inline]
    pub fn from_be_bytes(bytes: &[u8]) -> UBig {
        if bytes.len() <= WORD_BYTES {
            // fast path
            UBig::from_word(primitive::word_from_be_bytes_partial(bytes))
        } else {
            UBig::from_be_bytes_large(bytes)
        }
    }

    fn from_be_bytes_large(bytes: &[u8]) -> UBig {
        debug_assert!(bytes.len() > WORD_BYTES);
        let mut buffer = Buffer::allocate((bytes.len() - 1) / WORD_BYTES + 1);
        let mut chunks = bytes.rchunks_exact(WORD_BYTES);
        for chunk in &mut chunks {
            buffer.push(Word::from_be_bytes(chunk.try_into().unwrap()));
        }
        if !chunks.remainder().is_empty() {
            buffer.push(primitive::word_from_be_bytes_partial(chunks.remainder()));
        }
        buffer.into()
    }

    /// Return little-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert!(ubig!(0).to_le_bytes().is_empty());
    /// assert_eq!(ubig!(0x010203).to_le_bytes(), [3, 2, 1]);
    /// ```
    pub fn to_le_bytes(&self) -> Vec<u8> {
        match self.repr() {
            Small(x) => {
                let bytes = x.to_le_bytes();
                let skip_bytes = x.leading_zeros() as usize / 8;
                bytes[..WORD_BYTES - skip_bytes].to_vec()
            }
            Large(buffer) => {
                let n = buffer.len();
                let last = buffer[n - 1];
                let skip_last_bytes = last.leading_zeros() as usize / 8;
                let mut bytes = Vec::with_capacity(n * WORD_BYTES - skip_last_bytes);
                for word in &buffer[..n - 1] {
                    bytes.extend_from_slice(&word.to_le_bytes());
                }
                let last_bytes = last.to_le_bytes();
                bytes.extend_from_slice(&last_bytes[..WORD_BYTES - skip_last_bytes]);
                bytes
            }
        }
    }

    /// Return big-endian bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert!(ubig!(0).to_be_bytes().is_empty());
    /// assert_eq!(ubig!(0x010203).to_be_bytes(), [1, 2, 3]);
    /// ```
    pub fn to_be_bytes(&self) -> Vec<u8> {
        match self.repr() {
            Small(x) => {
                let bytes = x.to_be_bytes();
                let skip_bytes = x.leading_zeros() as usize / 8;
                bytes[skip_bytes..].to_vec()
            }
            Large(buffer) => {
                let n = buffer.len();
                let last = buffer[n - 1];
                let skip_last_bytes = last.leading_zeros() as usize / 8;
                let mut bytes = Vec::with_capacity(n * WORD_BYTES - skip_last_bytes);
                let last_bytes = last.to_be_bytes();
                bytes.extend_from_slice(&last_bytes[skip_last_bytes..]);
                for word in buffer[..n - 1].iter().rev() {
                    bytes.extend_from_slice(&word.to_be_bytes());
                }
                bytes
            }
        }
    }

    /// Convert to f32.
    ///
    /// Round to nearest, breaking ties to even last bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(134).to_f32(), 134.0f32);
    /// ```
    #[inline]
    pub fn to_f32(&self) -> f32 {
        match self.repr() {
            Small(word) => *word as f32,
            Large(_) => match u32::try_from(self) {
                Ok(val) => val as f32,
                Err(_) => self.to_f32_slow(),
            },
        }
    }

    fn to_f32_slow