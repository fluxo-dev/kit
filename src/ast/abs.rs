//! λ-abstraction, aka anonymous function, and related behaviors.

use super::{Binder, Exp, Idx, Sym};
use crate::err::SystemErr;

/// λ-abstraction, aka anonymous function, which maps one expression to another.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Abs {
    /// Original name of the variable that this binder was created with.
    pub sym: Sym,
    /// Type of the variable that this binder was created with.
    pub typ: Box<Exp>,
    /// Body or definition within the binder.
    pub exp: Box<Exp>,
}

impl Abs {
    /// Create a new instance of a [λ-abstraction][Abs].
    pub fn new(sym: Sym, typ: Exp, mut exp: Exp) -> Result<Self, SystemErr> {
        exp.index(&sym, &Idx::new(&sym))?;
        Ok(Self {
            sym,
            typ: Box::new(typ),
            exp: Box::new(exp),
        })
    }
}

impl Binder for Abs {
    fn prefix(&self) -> &'static str {
        "λ"
    }

    fn sym(&self) -> &Sym {
        &self.sym
    }

    fn typ(&self) -> &Exp {
        &self.typ
    }

    fn exp(&self) -> &Exp {
        &self.exp
    }
}
