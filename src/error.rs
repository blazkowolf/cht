use hyper::http::uri;
use std::fmt;

#[derive(Debug)]
pub enum ChtError {
    TooFewArguments,
    InvalidChtShUri(uri::InvalidUri),
}

impl fmt::Display for ChtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ChtError::InvalidChtShUri(ref err) => err.fmt(f),
            &ChtError::TooFewArguments => f.write_str("Not enough arguments! Must provide a programming language name and optionally a query string.")
        }
    }
}

impl From<uri::InvalidUri> for ChtError {
    fn from(error: uri::InvalidUri) -> Self {
        ChtError::InvalidChtShUri(error)
    }
}
