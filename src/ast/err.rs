//! Error types when working with an [expression][super::Exp].

use std::fmt::{Display, Formatter};

type Formatted = std::fmt::Result;

/// Error indicating an overflow that the system is not designed to handle.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OverflowErr {
    /// Message explaining the overflow.
    pub msg: String,
}

/// Error indicating a syntactical or semantic error decoding a value to an [expression][super::Exp].
pub struct DecodeErr {
    /// Message explaining the decoding error.
    pub msg: String,
}

impl Display for OverflowErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        write!(f, "{}", self.msg)
    }
}

impl Display for DecodeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        write!(f, "{}", self.msg)
    }
}
