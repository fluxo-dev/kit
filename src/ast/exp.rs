//! Top-level entity within the Abstract Syntax Tree (AST).

use super::{Abs, App, Idx, Prd, Sum, Sym, Unv, Var};
use crate::err::SystemErr;

/// Expression, which is the top-level entity within the AST.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Exp {
    /// Variable, which is the atomic constituent of an expression.
    Var(Var),
    /// Application, which denotes an operation to be performed on an expression.
    App(App),
    /// λ-abstraction, aka anonymous function, which maps one expression to another.
    Abs(Abs),
    /// Π-type, aka product type, which is a constructor for types.
    Prd(Prd),
    /// Σ-type, aka coproduct type, which is a constructor for types.
    Sum(Sum),
    /// Stratified type universe.
    Unv(Unv),
}

/// Binder that associates a [variable][super::var::Var] within an [expression][Exp].
pub trait Binder {
    /// Returns the prefix associated with this type of binder.
    fn prefix(&self) -> &'static str;
    /// Returns a reference to the symbol associated with the binder.
    fn sym(&self) -> &Sym;
    /// Returns a reference to the type of the symbol associated with the binder.
    fn typ(&self) -> &Exp;
    /// Returns a reference to the expression contained within this binder.
    fn exp(&self) -> &Exp;
}

impl Exp {
    /// Generates De Bruijn indices for this expression.
    ///
    /// Each free variable in the expression is compared with the supplied symbol, and converted
    /// into an index if it references the same symbol. If the scan traverses another binder, we
    /// increment the index value before continuing. This ensures that an index counts the number
    /// of binders in between its current position and the binder that binds it.
    pub fn index(&mut self, sym: &Sym, idx: &Idx) -> Result<(), SystemErr> {
        match self {
            Exp::Var(var) => {
                if let Var::Sym(can) = var {
                    if can == sym {
                        *var = Var::Idx(idx.clone()); // matches, so convert variable to index
                        Ok(())
                    } else {
                        Ok(()) // no match
                    }
                } else {
                    Ok(()) // variable is already bound (converted to an index)
                }
            }
            Exp::Abs(abs) => {
                let Abs { sym: can, exp, .. } = abs;
                if can != sym {
                    exp.index(sym, &idx.inc()?)?; // descend into nested expression
                } // otherwise short-circuit due to shadow binding
                Ok(())
            }
            Exp::App(app) => {
                let App { fst, snd } = app;
                fst.index(sym, idx)?; // branch and continue indexing
                snd.index(sym, idx)?; // branch and continue indexing
                Ok(())
            }
            Exp::Prd(prd) => {
                let Prd { sym: can, exp, .. } = prd;
                if can != sym {
                    exp.index(sym, &idx.inc()?)?; // descend into nested expression
                } // otherwise short-circuit due to shadow binding
                Ok(())
            }
            Exp::Sum(sum) => {
                let Sum { sym: can, exp, .. } = sum;
                if can != sym {
                    exp.index(sym, &idx.inc()?)?; // descend into nested expression
                } // otherwise short-circuit due to shadow binding
                Ok(())
            }
            Exp::Unv(_) => Ok(()), // constants need no indexing
        }
    }
}
