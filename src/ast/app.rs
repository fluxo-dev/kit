//! Application type, and related behaviors.

use super::Exp;

/// Application, which denotes an operation to be performed on an [expression][super::Exp].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct App {
    /// Expression denoting the operation to be performed.
    pub fst: Box<Exp>,
    /// Expression denoting the expression the operation is to be performed on.
    pub snd: Box<Exp>,
}

impl App {
    /// Create a new instance of an [application][App].
    pub fn new(fst: Exp, snd: Exp) -> Self {
        Self {
            fst: Box::new(fst),
            snd: Box::new(snd),
        }
    }
}
