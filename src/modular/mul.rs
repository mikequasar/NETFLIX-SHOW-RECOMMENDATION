use crate::{
    arch::word::Word,
    assert::debug_assert_in_const_fn,
    div,
    memory::{self, Memory, MemoryAllocation},
    modular::{
        modulo::{Modulo, ModuloLarge, ModuloRepr, ModuloSmall, ModuloSmallRaw},
        modulo_ring::{ModuloRingLarge, ModuloRingSmall},
    },
    