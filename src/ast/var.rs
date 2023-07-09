//! Variable, which is one of the atomic constituents of any [expression][super::Exp].

use crate::err::SystemErr;
use crate::fmt::Formatted;
use std::fmt::{Display, Formatter};

/// Variable, which is one of the atomic constituents of any [expression][super::Exp].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Var {
    /// Symbol that denotes the [variable][Var].
    Sym(Sym),
    /// De Bruijn index that denotes the [variable][Var] when bound within an [expression][super::Exp].
    Idx(Idx),
}

/// Name given to a [variable][Var].
///
/// A symbol is a name given to a [variable][Var]. Symbols exist because we
/// need a way to reference free variables in any given [expression][super::Exp]. Bound
/// variables track the symbols they were originally associated with, though this tracking has no
/// semantic significance.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Sym {
    /// Raw value of the name contained in this symbol.
    pub val: String,
}

/// De Bruijn index that denotes a [variable][Var] when bound within an [expression][super::Exp].
///
/// The De Bruijn index represents the number of binders between this bound variable and its parent
/// binder. Using the De Bruijn makes it easy to evaluate expressions without the need for complex,
/// α-substitution methods having to be applied. We support indexes up to [u64::MAX]; this gives us
/// an upper bound on the complexity of expressions that the system supports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Idx {
    /// Numeric value of this index.
    pub val: u64,
    /// Symbol of the bound variable that this index refers to.
    pub sym: Sym,
}

impl Sym {
    /// Create a new instance of a [symbol][Sym].
    pub fn new(val: &str) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Idx {
    /// Create a new instance of an index with value 0.
    pub fn new(sym: &Sym) -> Self {
        Self {
            val: 0,
            sym: sym.clone(),
        }
    }

    /// Create a new instance of an index with a higher value.
    pub fn inc(&self) -> Result<Self, SystemErr> {
        self.val
            .checked_add(1)
            .map(|val| Self {
                val,
                sym: self.sym.clone(),
            })
            .ok_or(SystemErr::MaxLimitIdx(self.val))
    }

    /// Create a new instance of an index with a lower value.
    ///
    /// <p style="background: rgba(255, 181, 77, 0.16); padding: 0.75em">
    /// <strong>Warning:</strong> This method panics when called if the current index value is 0.
    /// </p>
    pub fn dec(&self) -> Self {
        Self {
            val: self.val - 1,
            sym: self.sym.clone(),
        }
    }
}

impl From<Sym> for Var {
    fn from(sym: Sym) -> Self {
        Var::Sym(sym)
    }
}

impl From<Idx> for Var {
    fn from(idx: Idx) -> Self {
        Var::Idx(idx)
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        match self {
            Var::Idx(idx) => write!(f, "{}", idx),
            Var::Sym(sym) => write!(f, "{}", sym),
        }
    }
}

impl Display for Sym {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        write!(f, "{}", self.val)
    }
}

impl Display for Idx {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_inc() {
        let o1 = Idx::new(&Sym::new("foo"));
        assert!(o1.inc().is_ok());
        assert_eq!(o1.inc().unwrap().val, 1);
    }

    #[test]
    fn test_inc_overflow() {
        let o1 = Idx {
            val: u64::MAX - 1,
            sym: Sym::new("foo"),
        };
        assert!(o1.inc().is_ok());
        assert_eq!(o1.inc().unwrap().val, u64::MAX);

        let o2 = o1.inc().unwrap();
        assert!(o2.inc().is_err()); // overflow expected
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_dec_panic() {
        let o1 = Idx::new(&Sym::new("foo")).inc().unwrap();
        assert_eq!(o1.dec(), Idx::new(&Sym::new("foo")));

        let o2 = o1.dec();
        o2.dec(); // panic expected
    }

    #[test]
    fn test_display_sym() {
        let o1 = Sym::new("tangerine");
        assert_eq!(o1.to_string(), "tangerine");
    }

    #[test]
    fn test_display_idx() -> Result<(), SystemErr> {
        let o1 = Idx::new(&Sym::new("foo"));
        let o2 = o1.inc()?;
        let o3 = Idx {
            val: 3944,
            sym: Sym::new("foo"),
        };
        assert_eq!(o1.to_string(), "0");
        assert_eq!(o2.to_string(), "1");
        assert_eq!(o3.to_string(), "3944");
        Ok(())
    }
}
