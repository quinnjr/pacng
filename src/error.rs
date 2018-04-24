// Copyright (c) 2017-2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

extern crate toml;
use std::error::Error as StdError;
use std::{io, fmt};

pub enum ExitCode {
    ErrorNotRoot
}

///
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Toml(toml::de::Error)
}

///
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Toml(ref err) => write!(f, "TOML error: {}", err)
        }
    }
}

impl StdError for Error {
    ///
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Toml(ref err) => err.description()
        }
    }
    ///
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Toml(ref err) => Some(err)
        }
    }
}
