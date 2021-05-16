
//! Bitwise operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    helper_macros,
    ibig::IBig,
    math,
    ops::{AndNot, NextPowerOfTwo, UnsignedAbs},
    primitive::{double_word, PrimitiveSigned, PrimitiveUnsigned, WORD_BITS_USIZE},
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

impl UBig {
    /// Returns true if the `n`-th bit is set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(0b10010).bit(1), true);
    /// assert_eq!(ubig!(0b10010).bit(3), false);
    /// assert_eq!(ubig!(0b10010).bit(100), false);
    /// ```
    #[inline]
    pub fn bit(&self, n: usize) -> bool {
        match self.repr() {
            Small(word) => n < WORD_BITS_USIZE && word & 1 << n != 0,
            Large(buffer) => {
                let idx = n / WORD_BITS_USIZE;
                idx < buffer.len() && buffer[idx] & 1 << (n % WORD_BITS_USIZE) != 0
            }
        }
    }

    /// Set the `n`-th bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// let mut a = ubig!(0b100);
    /// a.set_bit(0);
    /// assert_eq!(a, ubig!(0b101));
    /// a.set_bit(10);
    /// assert_eq!(a, ubig!(0b10000000101));
    /// ```
    #[inline]
    pub fn set_bit(&mut self, n: usize) {
        match mem::take(self).into_repr() {
            Small(word) => {
                if n < WORD_BITS_USIZE {
                    *self = UBig::from_word(word | 1 << n)
                } else {
                    *self = UBig::with_bit_word_slow(word, n)
                }
            }
            Large(buffer) => *self = UBig::with_bit_large(buffer, n),
        }
    }

    fn with_bit_word_slow(word: Word, n: usize) -> UBig {
        debug_assert!(n >= WORD_BITS_USIZE);
        let idx = n / WORD_BITS_USIZE;
        let mut buffer = Buffer::allocate(idx + 1);
        buffer.push(word);
        buffer.extend((1..idx).map(|_| 0));
        buffer.push(1 << (n % WORD_BITS_USIZE));
        buffer.into()
    }

    fn with_bit_large(mut buffer: Buffer, n: usize) -> UBig {
        let idx = n / WORD_BITS_USIZE;
        if idx < buffer.len() {
            buffer[idx] |= 1 << (n % WORD_BITS_USIZE);
        } else {
            buffer.ensure_capacity(idx + 1);
            buffer.push_zeros(idx - buffer.len());
            buffer.push(1 << (n % WORD_BITS_USIZE));
        }
        buffer.into()
    }

    /// Clear the `n`-th bit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// let mut a = ubig!(0b101);
    /// a.clear_bit(0);
    /// assert_eq!(a, ubig!(0b100));
    /// ```
    #[inline]
    pub fn clear_bit(&mut self, n: usize) {
        match mem::take(self).into_repr() {
            Small(word) => {
                if n < WORD_BITS_USIZE {
                    *self = UBig::from_word(word & !(1 << n))
                }
            }
            Large(buffer) => *self = UBig::without_bit_large(buffer, n),
        }
    }

    fn without_bit_large(mut buffer: Buffer, n: usize) -> UBig {
        let idx = n / WORD_BITS_USIZE;
        if idx < buffer.len() {
            buffer[idx] &= !(1 << (n % WORD_BITS_USIZE));
        }
        buffer.into()
    }

    /// Returns the number of trailing zeros in the binary representation.
    ///
    /// In other words, it is the largest `n` such that 2 to the power of `n` divides the number.
    ///
    /// For 0, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(17).trailing_zeros(), Some(0));
    /// assert_eq!(ubig!(48).trailing_zeros(), Some(4));
    /// assert_eq!(ubig!(0b101000000).trailing_zeros(), Some(6));
    /// assert_eq!(ubig!(0).trailing_zeros(), None);
    /// ```
    #[inline]
    pub fn trailing_zeros(&self) -> Option<usize> {
        match self.repr() {
            Small(0) => None,
            Small(word) => Some(word.trailing_zeros() as usize),
            Large(buffer) => Some(UBig::trailing_zeros_large(buffer)),
        }
    }

    fn trailing_zeros_large(words: &[Word]) -> usize {
        debug_assert!(*words.last().unwrap() != 0);

        for (idx, word) in words.iter().enumerate() {
            if *word != 0 {
                return idx * WORD_BITS_USIZE + word.trailing_zeros() as usize;
            }
        }
        panic!("trailing_zeros_large(0)")
    }

    /// Bit length.
    ///
    /// The length of the binary representation of the number.
    ///
    /// For 0, the length is 0.
    ///
    /// For non-zero numbers it is:
    /// * `in_radix(2).to_string().len()`
    /// * the index of the top 1 bit plus one
    /// * the floor of the logarithm base 2 of the number plus one.
    ///
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(17).bit_len(), 5);
    /// assert_eq!(ubig!(0b101000000).bit_len(), 9);
    /// assert_eq!(ubig!(0).bit_len(), 0);
    /// let x = ubig!(_0x90ffff3450897234);
    /// assert_eq!(x.bit_len(), x.in_radix(2).to_string().len());
    /// ```
    #[inline]
    pub fn bit_len(&self) -> usize {
        match self.repr() {
            Small(word) => math::bit_len(*word) as usize,
            Large(buffer) => {
                buffer.len() * WORD_BITS_USIZE - buffer.last().unwrap().leading_zeros() as usize
            }
        }
    }

    /// True if the number is a power of 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ubig;
    /// assert_eq!(ubig!(0).is_power_of_two(), false);
    /// assert_eq!(ubig!(8).is_power_of_two(), true);
    /// assert_eq!(ubig!(9).is_power_of_two(), false);
    /// ```
    #[inline]
    pub fn is_power_of_two(&self) -> bool {
        match self.repr() {
            Small(word) => word.is_power_of_two(),
            Large(buffer) => UBig::is_power_of_two_large(buffer),
        }
    }

    fn is_power_of_two_large(words: &[Word]) -> bool {
        debug_assert!(*words.last().unwrap() != 0);

        words[..words.len() - 1].iter().all(|x| *x == 0) && words.last().unwrap().is_power_of_two()
    }
}

impl IBig {
    /// Returns the number of trailing zeros in the two's complement binary representation.
    ///
    /// In other words, it is the largest `n` such that 2 to the power of `n` divides the number.
    ///
    /// For 0, it returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ibig::ibig;
    /// assert_eq!(ibig!(17).trailing_zeros(), Some(0));
    /// assert_eq!(ibig!(-48).trailing_zeros(), Some(4));
    /// assert_eq!(ibig!(-0b101000000).trailing_zeros(), Some(6));
    /// assert_eq!(ibig!(0).trailing_zeros(), None);
    /// ```
    #[inline]
    pub fn trailing_zeros(&self) -> Option<usize> {
        self.magnitude().trailing_zeros()
    }
}

impl NextPowerOfTwo for UBig {
    type Output = UBig;

    #[inline]
    fn next_power_of_two(self) -> UBig {
        match self.into_repr() {
            Small(word) => match word.checked_next_power_of_two() {
                Some(p) => UBig::from_word(p),
                None => UBig::from(double_word(0, 1)),
            },
            Large(buffer) => UBig::next_power_of_two_large(buffer),
        }
    }
}

impl NextPowerOfTwo for &UBig {
    type Output = UBig;

    #[inline]
    fn next_power_of_two(self) -> UBig {
        self.clone().next_power_of_two()
    }
}

impl UBig {
    fn next_power_of_two_large(mut buffer: Buffer) -> UBig {
        debug_assert!(*buffer.last().unwrap() != 0);

        let n = buffer.len();
        let mut iter = buffer[..n - 1].iter_mut().skip_while(|x| **x == 0);

        let carry = match iter.next() {
            None => 0,
            Some(x) => {
                *x = 0;
                for x in iter {
                    *x = 0;
                }
                1
            }
        };

        let last = buffer.last_mut().unwrap();
        match last
            .checked_add(carry)
            .and_then(|x| x.checked_next_power_of_two())
        {
            Some(p) => *last = p,
            None => {
                *last = 0;
                buffer.ensure_capacity(n + 1);
                buffer.push(1);
            }
        }

        buffer.into()
    }
}

impl BitAnd<UBig> for UBig {
    type Output = UBig;

    #[inline]
    fn bitand(self, rhs: UBig) -> UBig {
        match (self.into_repr(), rhs.into_repr()) {
            (Small(word0), Small(word1)) => UBig::from_word(word0 & word1),
            (Small(word0), Large(buffer1)) => UBig::from_word(word0 & buffer1.first().unwrap()),
            (Large(buffer0), Small(word1)) => UBig::from_word(buffer0.first().unwrap() & word1),
            (Large(buffer0), Large(buffer1)) => {