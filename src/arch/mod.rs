//! Architecture dependent functionality.

use cfg_if::cfg_if;

pub(crate) use arch_impl::add;
pub(crate) use arch_impl::digits;
pub(crate) use arch_impl::ntt;
pub(crate) use arch_impl::word;

// Architecture choice. The logic works like this:
// 1. If the configuration option force_bits is set to 16, 32 or 64, use generic_<n>_bit.
// 2. Otherwise if target_arch is known, select that architecture.
// 3. Otherwise target_pointer_width is 16 or 32, use generic_<n>_bit.
// 4. Otherwise, use generic_64_bit.
cfg_if! {
    // Step 1. Check force_bits.
    if #[cfg(force_bits = "16")] {
        #[path = "generic_16_bit/mod.rs"]
        mod arch_impl;
   