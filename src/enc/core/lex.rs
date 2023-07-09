//! Lexing utilities for the core language.

use crate::err::DecodeErr;
use crate::fmt::Formatted;
use logos::{Logos, SpannedIter};
use std::fmt::{Display, Formatter};

/// Tokens available to the grammar of the core language.
#[derive(Logos, Clone, Debug, Eq, Hash, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Tok {
    /// Identifier token.
    ///
    /// An identifier token starts with exactly one lowercase alphabet, and is followed by zero or
    /// more lowercase alphabets, decimal numbers or underscores. An identifier cannot be empty.
    #[regex("[a-z][a-z0-9_]*", |lex| lex.slice().parse().ok())]
    Ident(String),
    /// Left parenthesis token.
    #[token("(")]
    LParen,
    /// Right parenthesis token.
    #[token(")")]
    RParen,
    /// Dot (period) token.
    #[token(".")]
    Dot,
    /// Colon token.
    #[token(":")]
    Colon,
    /// Lowercase Greek letter *lambda* token.
    #[token("λ")]
    Lambda,
    /// Uppercase Greek letter *pi* token.
    #[token("Π")]
    Pi,
    /// Uppercase Greek letter *sigma* token.
    #[token("Σ")]
    Sigma,
    /// Box character token.
    #[token("□")]
    Box,
}

impl Display for Tok {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        match self {
            Tok::Ident(ident) => write!(f, "{}", ident),
            Tok::LParen => write!(f, "("),
            Tok::RParen => write!(f, ")"),
            Tok::Dot => write!(f, "."),
            Tok::Colon => write!(f, ":"),
            Tok::Lambda => write!(f, "λ"),
            Tok::Pi => write!(f, "Π"),
            Tok::Sigma => write!(f, "Σ"),
            Tok::Box => write!(f, "□"),
        }
    }
}

pub struct Lexer<'input> {
    /// Stream of tokens, where each token is paired with its location in the source input stream.
    token_stream: SpannedIter<'input, Tok>,
}

impl<'input> Lexer<'input> {
    /// Create a new instance of the lexer.
    pub fn new(input: &'input str) -> Self {
        Self {
            token_stream: Tok::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Tok, usize), DecodeErr>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(res, span)| match res {
            Ok(tok) => Ok((span.start, tok, span.end)),
            Err(()) => Err(DecodeErr::InvalidToken(span.start)),
        })
    }
}
