// Copyright 2022 Vladislav Melnik
// SPDX-License-Identifier: MIT

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

use nom::{
    error::{ErrorKind as NomErrorKind, FromExternalError, ParseError as NomParseError},
    Err,
};

use super::limit::LimitError;

#[derive(Debug)]
pub enum ParseErrorKind {
    Nom(NomErrorKind),
    Limit(LimitError, &'static str),
    UnknownTag { tag: String, hint: &'static str },
    Custom(NomErrorKind, String),
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::Nom(err) => write!(f, "{err:?}"),
            ParseErrorKind::Limit(err, hint) => write!(f, "{hint}, {err}"),
            ParseErrorKind::UnknownTag { tag, hint } => write!(f, "unknown tag: {tag}, {hint}"),
            ParseErrorKind::Custom(err, custom) => write!(f, "{err:?}, custom: {custom}"),
        }
    }
}

impl ParseErrorKind {
    pub fn unknown_tag<T>(tag: T, hint: &'static str) -> Self
    where
        T: fmt::Debug,
    {
        ParseErrorKind::UnknownTag {
            tag: format!("{tag:?}"),
            hint,
        }
    }

    pub fn error<I>(self, input: I) -> Err<ParseError<I>> {
        Err::Error(ParseError {
            input,
            kind: self,
            subsequent: None,
        })
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, ParseErrorKind::Nom(nom::error::ErrorKind::Eof))
    }
}

pub struct ParseError<I> {
    pub input: I,
    pub kind: ParseErrorKind,
    pub subsequent: Option<Box<ParseError<I>>>,
}

impl<I> fmt::Display for ParseError<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = &self.kind;
        match &self.subsequent {
            None => write!(f, "kind: {kind}"),
            Some(ref subsequent) => write!(f, "kind: {kind}, subsequent: ({subsequent})"),
        }
    }
}

impl<I> fmt::Debug for ParseError<I>
where
    I: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("subsequent", &self.subsequent)
            .field("input", &hex::encode(&self.input))
            .finish()
    }
}

impl<I> ParseError<I>
where
    I: Into<Vec<u8>>,
{
    pub fn into_vec(self) -> ParseError<Vec<u8>> {
        ParseError {
            input: self.input.into(),
            kind: self.kind,
            subsequent: self.subsequent.map(|e| Box::new(ParseError::into_vec(*e))),
        }
    }
}

impl<I> NomParseError<I> for ParseError<I> {
    fn from_error_kind(input: I, kind: NomErrorKind) -> Self {
        ParseError {
            input,
            kind: ParseErrorKind::Nom(kind),
            subsequent: None,
        }
    }

    fn append(input: I, kind: NomErrorKind, other: Self) -> Self {
        ParseError {
            input,
            kind: ParseErrorKind::Nom(kind),
            subsequent: Some(Box::new(other)),
        }
    }
}

impl<I, E> FromExternalError<I, E> for ParseError<I>
where
    E: fmt::Display,
{
    fn from_external_error(input: I, kind: NomErrorKind, e: E) -> Self {
        ParseError {
            input,
            kind: ParseErrorKind::Custom(kind, e.to_string()),
            subsequent: None,
        }
    }
}
