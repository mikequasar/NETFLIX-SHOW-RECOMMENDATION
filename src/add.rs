
//! Addition and subtraction functions.

use crate::{
    arch::{
        self,
        word::{SignedWord, Word},
    },
    primitive::PrimitiveSigned,
    sign::Sign::{self, *},
};
use core::cmp::Ordering::*;

/// Add one to a word sequence.
///
/// Returns overflow.
#[must_use]
pub(crate) fn add_one_in_place(words: &mut [Word]) -> bool {
    for word in words {
        let (a, overflow) = word.overflowing_add(1);
        *word = a;
        if !overflow {
            return false;
        }
    }
    true
}

/// Subtract one from a word sequence.
///
/// Returns borrow.
#[must_use]
pub(crate) fn sub_one_in_place(words: &mut [Word]) -> bool {
    for word in words {
        let (a, borrow) = word.overflowing_sub(1);
        *word = a;
        if !borrow {
            return false;
        }
    }
    true
}

/// Add a word to a non-empty word sequence.
///
/// Returns overflow.
#[must_use]
pub(crate) fn add_word_in_place(words: &mut [Word], rhs: Word) -> bool {
    let (word_0, words_hi) = words.split_first_mut().unwrap();
    let (a, overflow) = word_0.overflowing_add(rhs);
    *word_0 = a;
    overflow && add_one_in_place(words_hi)
}

/// Subtract a word from a non-empty word sequence.
///
/// Returns borrow.
#[must_use]
pub(crate) fn sub_word_in_place(words: &mut [Word], rhs: Word) -> bool {
    let (word_0, words_hi) = words.split_first_mut().unwrap();
    let (a, borrow) = word_0.overflowing_sub(rhs);
    *word_0 = a;
    borrow && sub_one_in_place(words_hi)
}

/// Add a word sequence of same length in place.
///
/// Returns overflow.