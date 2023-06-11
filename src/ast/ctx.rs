//! Typing context, and related behaviors.

/// Typing context.
///
/// A typing context (represented by the `Γ` symbol) is an ordered set of declarations of the form
/// `X : N`, with `X` a [variable][super::Exp::Var], `N` an [expression][super::Exp] that
/// denotes the type of `X`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ctx;
