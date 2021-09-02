//! Utility functions for writing [`nom`] parser tests.

use nom::error::{Error, ErrorKind};

/// Syntax sugar to `nom::Err::Error(nom::error::Error::new(input, code))`.
#[cfg(test)]
pub fn parse_error<I>(input: I, code: ErrorKind) -> nom::Err<Error<I>> {
    nom::Err::Error(Error::new(input, code))
}
