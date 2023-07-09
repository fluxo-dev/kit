//! Encodings, aka languages, supported for the Abstract Syntax Tree (AST).

use crate::ast::Exp;
use crate::err::DecodeErr;

/// Trait that maps an [expression][Exp] from and to an encoding of type `T`.
pub trait Codec<T> {
    /// Encode an [expression][Exp] to an object of type `T`.
    fn encode(&self, exp: &Exp) -> T;

    /// Decode an value of type `T` to an [expression][Exp].
    fn decode(&self, val: &T) -> Result<Exp, DecodeErr>;
}

pub mod core;
