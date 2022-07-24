//! Bit shift operators.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    ibig::IBig,
    primitive::{double_word, extend_word, split_double_word, WORD_BITS_USIZE},
    shift,
    sign::Sign::*,
    ubig::{Repr::*, UBig},
};
use core::{
    mem,
    ops::{Shl, ShlAss