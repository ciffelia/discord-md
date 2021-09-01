//! Utility functions for writing parser tests.

use nom::error::{ErrorKind, Error};

/// Syntax sugar to `nom::Err::Error(nom::error::Error::new(input, code))`.
///
/// Used in tests.
#[cfg(test)]
pub fn parse_error<I>(input: I, code: ErrorKind) -> nom::Err<Error<I>> {
    nom::Err::Error(Error::new(input, code))
}
