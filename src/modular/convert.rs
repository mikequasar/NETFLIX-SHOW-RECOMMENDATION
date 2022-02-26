
//! Conversion between Modulo, UBig and IBig.

use crate::{
    arch::word::Word,
    buffer::Buffer,
    div,
    ibig::IBig,
    memory::MemoryAllocation,
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::{ModuloRing, ModuloRingLarge, ModuloRingRepr, ModuloRingSmall},
    },
    primitive::extend_word,
    shift,
    sign::Sign::*,
    ubig::{Repr, UBig},
};
use alloc::vec::Vec;
use core::iter;