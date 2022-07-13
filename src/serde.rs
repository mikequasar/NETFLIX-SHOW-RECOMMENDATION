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
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let chunks = self.as_words().chunks(WORDS_PER_U64);
        let mut seq = serializer.serialize_seq(Some(chunks.len()))?;
        for chunk in chunks {
            let mut word_u64: u64 = 0;
            for (i, word) in chunk.iter().enumerate() {
                word_u64 |= u64::from(*word) << (i * WORD_BITS_USIZE);
            }
            seq.serialize_element(&word_u64)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for UBig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(UBigVisitor)
    }
}

struct UBigVisitor;

impl<'de> Visitor<'de> for UBigVisitor {
    type Value = UBig;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "a sequence of 64-bit words")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<UBig