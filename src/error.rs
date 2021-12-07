//! Error types.

use core::fmt::{self, Display, Formatter};

/// Number out of bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OutOfBoundsError;

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
  