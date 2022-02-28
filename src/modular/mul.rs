use crate::{
    arch::word::Word,
    assert::debug_assert_in_const_fn,
    div,
    memory::{self, Memory, MemoryAllocation},
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::{ModuloRingLarge, ModuloRingSmall},
    },
    mul,
    primitive::extend_word,
    shift,
    sign::Sign::Positive,
};
use alloc::alloc::Layout;
use core::ops::{Mul, MulAssign};

impl<'a> Mul<Modulo<'a>> for Modulo<'a> {
    type Output =