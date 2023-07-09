//! Parsing utilities for the core language.

lalrpop_mod!(pub grammar, "/enc/core/grammar.rs");

use super::lex::Lexer;
use crate::ast::{App, Binder, Exp, Var};
use crate::enc::Codec;
use crate::err::DecodeErr;
use grammar::ExpParser;

/// Core language implementing the *canonical* encoding of the Abstract Syntax Tree (AST).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Core {
    /// Flag that indicates that the current traversal is *exclusively* within the left sub-tree of the AST.
    ltree: bool,
    /// Flag that indicates that the current traversal is *exclusively* within the right sub-tree of the AST.
    rtree: bool,
    /// Show bound variables as De Bruijn indices rather than their original symbols.
    show_indices: bool,
}

impl Core {
    /// Create a new instance of the codec.
    pub fn new() -> Self {
        Self {
            ltree: false,
            rtree: false,
            show_indices: false,
        }
    }

    pub fn reset(&self) -> Self {
        Self {
            ltree: false,
            rtree: false,
            ..*self
        }
    }

    /// Create a new instance of the codec with a value for `show_indices`.
    ///
    /// If the value is set to true, bound variables are rendered as De Bruijn indices. Otherwise,
    /// they are rendered as their original symbols.
    pub fn with_show_indices(show_indices: bool) -> Self {
        let mut res = Self::new();
        res.show_indices = show_indices;
        res
    }
}

impl Core {
    /// Format a [binder][Binder] object.
    fn fmt_binder<T: Binder>(&self, obj: &T) -> String {
        let func = || {
            format!(
                "{}{} : {} . {}",
                obj.prefix(),
                obj.sym(),
                self.reset().encode(obj.typ()), // not ambiguous, so start with a new branch
                self.reset().encode(obj.exp()), // greedy, so reset, start with a new branch
            )
        };
        self.fmt_parens(self.ltree, func) // parenthesize if on left sub-tree (exclusively)
    }

    /// Format an [application][App].
    fn fmt_app(&self, app: &App) -> String {
        let func = || {
            let ltree_codec = Self {
                ltree: !self.rtree, // true || reset if current term is being parenthesized
                ..*self
            };
            let rtree_codec = Self {
                rtree: !self.rtree, // true || reset if current term is being parenthesized
                ..*self
            };
            format!(
                "{} {}",
                ltree_codec.encode(&app.fst),
                rtree_codec.encode(&app.snd)
            )
        };
        self.fmt_parens(self.rtree, func) // parenthesize if on right sub-tree (exclusively)
    }

    /// Optionally parenthesizes an encoded [expression][Exp] formed with the supplied function, to
    /// create the encoded version of the expression.
    fn fmt_parens<F>(&self, parens: bool, func: F) -> String
    where
        F: FnOnce() -> String,
    {
        if parens {
            format!("({})", func())
        } else {
            func()
        }
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Codec<String> for Core {
    fn encode(&self, exp: &Exp) -> String {
        match exp {
            Exp::Var(var) => match var {
                Var::Sym(sym) => format!("{}", sym),
                Var::Idx(idx) => {
                    if self.show_indices {
                        idx.val.to_string()
                    } else {
                        idx.sym.to_string()
                    }
                }
            },
            Exp::App(app) => self.fmt_app(app),
            Exp::Abs(abs) => self.fmt_binder(abs),
            Exp::Prd(prd) => self.fmt_binder(prd),
            Exp::Sum(sum) => self.fmt_binder(sum),
            Exp::Unv(unv) => format!("{}", unv),
        }
    }

    fn decode(&self, val: &String) -> Result<Exp, DecodeErr> {
        ExpParser::new()
            .parse(Lexer::new(val.as_str()))
            .map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn check(err: &mut Vec<String>, items: &Vec<&str>) {
        for val in items {
            Core::new()
                .decode(&val.to_string())
                .map(|exp| Core::new().encode(&exp))
                .map(|can| {
                    if val != &can.as_str() {
                        err.push(format!("assertion failed: {} != {}", val, can))
                    }
                })
                .map_err(|e| {
                    err.push(format!(
                        "decoding failed for expression: {}, error: {}",
                        val, e
                    ))
                })
                .unwrap_or_default();
        }
    }

    #[test]
    fn test_decode_encode() {
        let mut err = vec![];
        let items = vec![
            "foo",
            "foo bar",
            "foo (bar moo)",
            "λbar : float . λmoo : char . λfoo : int . foo (bar moo)",
            "λfoo : int . foo (bar moo)",
            "λbar : char . λfoo : int . foo (bar moo)",
            "λbar : Πf : int . f . λmoo : char . λfoo : int . foo (bar moo)",
            "λfoo : Πf : int . f . foo (bar moo)",
            "λbar : Πf : char . f . λfoo : int . foo (bar moo)",
            "λbar : Σf : int . f . λmoo : char . λfoo : int . foo (bar moo)",
            "λfoo : Σf : int . f . foo (bar moo)",
            "λbar : Σf : char . f . λfoo : int . foo (bar moo)",
            "foo λbar : int . bar moo",
            "(λfoo : □ . bar) λmoo : □ . moo",
        ];
        check(&mut err, &items);
        assert!(err.is_empty(), "checks failed:\n{}", err.join("\n"));
    }
}
