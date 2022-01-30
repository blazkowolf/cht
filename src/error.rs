use hyper::http::uri;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ChtError {
    TooFewArguments,
    InvalidChtShUri(uri::InvalidUri),
}

impl error::Error for ChtError {
    fn description(&self) -> &str {
        match *self {
            ChtError::InvalidChtShUri(_) => "Not a valid cheat.sh URI",
            ChtError::TooFewArguments => "Not enough arguments! Must provide a programming language name and optionally a query string.",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ChtError::InvalidChtShUri(ref err) => Some(err),
            ChtError::TooFewArguments => None,
        }
    }
}

impl fmt::Display for ChtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ChtError::InvalidChtShUri(ref err) => err.fmt(f),
            ChtError::TooFewArguments => f.write_str("TooFewArguments"),
        }
    }
}

impl From<uri::InvalidUri> for ChtError {
    fn from(error: uri::InvalidUri) -> Self {
        ChtError::InvalidChtShUri(error)
    }
}
