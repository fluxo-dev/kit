//! Top-level error types.

use crate::enc::core::lex::Tok;
use crate::fmt::Formatted;
use lalrpop_util::ParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error indicating a situation that the system is not designed to handle.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SystemErr {
    /// Maximum limit for [indices][crate::ast::Idx] associated with bound variables has been reached.
    MaxLimitIdx(u64),
    /// Maximum limit for [universe][crate::ast::Unv] levels has been reached.
    MaxLimitUnv(u64),
}

/// Error indicating a syntactic or semantic error decoding a value to an [expression][crate::ast::Exp].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DecodeErr {
    /// Additional tokens were expected by the grammar but the stream ended abruptly.
    EndOfStream(usize, Vec<String>),
    /// Token is not valid according to the lexer.
    InvalidToken(usize),
    /// Valid token was found but a different one (or none at all) was expected.
    UnexpectedToken(Tok, usize, usize, Vec<String>),
    /// A [SystemErr] was encountered while decoding the value.
    SystemErr(SystemErr),
}

impl Error for SystemErr {}
impl Error for DecodeErr {}

impl Display for SystemErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        match self {
            SystemErr::MaxLimitIdx(lim) => {
                write!(f, "max limit {} for indices has been reached", lim)
            }
            SystemErr::MaxLimitUnv(lim) => {
                write!(f, "max limit {} for universe levels has been reached", lim)
            }
        }
    }
}

impl Display for DecodeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Formatted {
        match self {
            DecodeErr::EndOfStream(sloc, expected) => write!(
                f,
                "unexpected end of stream, at location: {}, expected: {}",
                sloc,
                expected.join(" | ")
            ),
            DecodeErr::InvalidToken(sloc) => write!(f, "invalid token, at location {}", sloc),
            DecodeErr::UnexpectedToken(tok, sloc, eloc, expected) => write!(
                f,
                "unexpected token: {}, at location: {}..{}, expected: {}",
                tok,
                sloc,
                eloc,
                if expected.is_empty() {
                    "none".to_string()
                } else {
                    expected.join(" | ")
                }
            ),
            DecodeErr::SystemErr(err) => write!(f, "{}", err),
        }
    }
}

impl From<ParseError<usize, Tok, DecodeErr>> for DecodeErr {
    fn from(err: ParseError<usize, Tok, DecodeErr>) -> Self {
        match err {
            // Token is not valid according to the lexer.
            ParseError::InvalidToken { location: sloc } => DecodeErr::InvalidToken(sloc),
            // Additional tokens were expected by the grammar but the stream ended abruptly.
            ParseError::UnrecognizedEof {
                location: sloc,
                expected,
            } => DecodeErr::EndOfStream(sloc, expected),
            // Valid token was found but a different one was expected.
            ParseError::UnrecognizedToken {
                token: (sloc, tok, eloc),
                expected,
            } => DecodeErr::UnexpectedToken(tok, sloc, eloc, expected),
            // Valid token was found but none was expected.
            ParseError::ExtraToken {
                token: (sloc, tok, eloc),
            } => DecodeErr::UnexpectedToken(tok, sloc, eloc, vec![]),
            // Token is not valid according to the lexer, or the expression could not be constructed due to a system error.
            ParseError::User { error } => error,
        }
    }
}
