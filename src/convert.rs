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

    fn to_f32_slow(&self) -> f32 {
        let n = self.bit_len();
        debug_assert!(n > 32);

        if n > 128 {
            f32::INFINITY
        } else {
            let exponent = (n - 1) as u32;
            debug_assert!((32..128).contains(&exponent));
            let mantissa25 = u32::try_from(self >> (n - 25)).unwrap();
            let mantissa = mantissa25 >> 1;

            // value = [8 bits: exponent + 127][23 bits: mantissa without the top bit]
            let value = ((exponent + 126) << 23) + mantissa;

            // Calculate round-to-even adjustment.
            let extra_bit = self.are_low_bits_nonzero(n - 25);
            // low bit of mantissa and two extra bits
            let low_bits = ((mantissa25 & 0b11) << 1) | u32::from(extra_bit);
            let adjustment = round_to_even_adjustment(low_bits);

            // If adjustment is true, increase the mantissa.
            // If the mantissa overflows, this correctly increases the exponent and
            // sets the mantissa to 0.
            // If the exponent overflows, we correctly get the representation of infinity.
            let value = value + u32::from(adjustment);
            f32::from_bits(value)
        }
    }

    /// Convert to f64.
    ///
    /// Round to nearest, breaking ties to even last bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(134).to_f64(), 134.0f64);
    /// ```
    #[inline]
    pub fn to_f64(&self) -> f64 {
        match self.repr() {
            Small(word) => *word as f64,
            Large(_) => match u64::try_from(self) {
                Ok(val) => val as f64,
                Err(_) => self.to_f64_slow(),
            },
        }
    }

    fn to_f64_slow(&self) -> f64 {
        let n = self.bit_len();
        debug_assert!(n > 64);

        if n > 1024 {
            f64::INFINITY
        } else {
            let exponent = (n - 1) as u64;
            debug_assert!((64..1024).contains(&exponent));
            let mantissa54 = u64::try_from(self >> (n - 54)).unwrap();
            let mantissa = mantissa54 >> 1;

            // value = [11-bits: exponent + 1023][52 bit: mantissa without the top bit]
            let value = ((exponent + 1022) << 52) + mantissa;

            // Calculate round-to-even adjustment.
            let extra_bit = self.are_low_bits_nonzero(n -