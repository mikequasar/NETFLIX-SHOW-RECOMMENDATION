//! Word buffer.

use crate::{arch::word::Word, ubig::UBig};

use alloc::vec::Vec;
use core::{
    iter,
    ops::{Deref, DerefMut},
};

/// Buffer for Words.
///
/// UBig operations are usually performed by creating a Buffer with appropriate capacity, filling it
/// in with Words, and then converting to UBig.
///
/// If its capacity is exceeded, the `Buffer` will panic.
#[derive(Debug, Eq, Hash, PartialEq)]
pub(crate) struct Buffer(Vec<Word>);

impl Buffer