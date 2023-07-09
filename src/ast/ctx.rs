//! Typing context, and related behaviors.

/// Typing context.
///
/// A typing context (represented by the symbol `Γ`) is an ordered set of declarations of the form
/// `x : N`, `x` being a [variable][super::Exp::Var], and `N` an [expression][super::Exp] denoting
/// the type of `x`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ctx;
