use hyper::{http, http::uri};
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ChtError {
    InvalidChtShUri(uri::InvalidUri),
    UnknownCheatSheet,
    InvalidChtRequest(http::Error),
    Error(hyper::Error),
}

impl error::Error for ChtError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ChtError::InvalidChtShUri(ref err) => Some(err),
            ChtError::UnknownCheatSheet => None,
            ChtError::InvalidChtRequest(ref err) => Some(err),
            ChtError::Error(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ChtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ChtError::InvalidChtShUri(ref _err) => f.write_str("invalid cheat.sh URL provided"),
            ChtError::UnknownCheatSheet => f.write_str("unknown cheat sheet provided"),
            ChtError::InvalidChtRequest(ref err) => err.fmt(f),
            ChtError::Error(ref err) => err.fmt(f),
        }
    }
}

impl From<uri::InvalidUri> for ChtError {
    fn from(error: uri::InvalidUri) -> Self {
        ChtError::InvalidChtShUri(error)
    }
}

impl From<http::Error> for ChtError {
    fn from(error: http::Error) -> Self {
        ChtError::InvalidChtRequest(error)
    }
}

impl From<hyper::Error> for ChtError {
    fn from(error: hyper::Error) -> Self {
        ChtError::Error(error)
    }
}
