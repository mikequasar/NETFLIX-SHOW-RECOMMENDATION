//! Architecture dependent functionality.

use cfg_if::cfg_if;

pub(crate) use arch_impl::add;
pub(crate) use arch_impl::digits;
pub(crate) use arch_impl::ntt;
pub(crate) use arch_impl::word;

// Architecture choice. The logic works lik