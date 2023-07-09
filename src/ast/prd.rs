//! Π-type, aka product type, and related behaviors.

use super::{Binder, Exp, Idx, Sym};
use crate::err::SystemErr;

/// Π-type, aka product type, which is a constructor for types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Prd {
    /// Original name of the variable that this binder was created with.
    pub sym: Sym,
    /// Type of the variable that this binder was created with.
    pub typ: Box<Exp>,
    /// Body or definition within the binder.
    pub exp: Box<Exp>,
}

impl Prd {
    /// Create a new instance of a [Π-type][Prd].
    pub fn new(sym: Sym, typ: Exp, mut exp: Exp) -> Result<Self, SystemErr> {
        exp.index(&sym, &Idx::new(&sym))?;
        Ok(Self {
            sym,
            typ: Box::new(typ),
            exp: Box::new(exp),
        })
    }
}

impl Binder for Prd {
    fn prefix(&self) -> &'static str {
        "Π"
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
