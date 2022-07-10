use crate::{arch::word::Word, buffer::Buffer, ibig::IBig, primitive::WORD_BITS_USIZE, ubig::UBig};
use alloc::vec::Vec;
use core::fmt::{self, Formatter};
use serde::{
    de::{Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq, Serializer},
};
use static_assertions::const_assert;

const_assert!(64 % WORD_BITS_USIZE == 0);
const WORDS_PER_U64: usize = 64 / WORD_BITS_USIZE;

impl Serialize for UBig {
    #[allow(clippy::useless_conversion)]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>