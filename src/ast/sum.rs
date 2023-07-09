//! Σ-type, aka coproduct type, and related behaviors.

use super::{Binder, Exp, Idx, Sym};
use crate::err::SystemErr;

/// Σ-type, aka coproduct type, which is a constructor for types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Sum {
    /// Original name of the variable that this binder was created with.
    pub sym: Sym,
    /// Type of the variable that this binder was created with.
    pub typ: Box<Exp>,
    /// Body or definition within the binder.
    pub exp: Box<Exp>,
}

impl Sum {
    /// Create a new instance of a [Σ-type][Sum].
    pub fn new(sym: Sym, typ: Exp, mut exp: Exp) -> Result<Self, SystemErr> {
        exp.index(&sym, &Idx::new(&sym))?;
        Ok(Self {
            sym,
            typ: Box::new(typ),
            exp: Box::new(exp),
        })
    }
}

impl Binder for Sum {
    fn prefix(&self) -> &'static str {
        "Σ"
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
