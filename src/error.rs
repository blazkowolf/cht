use hyper::http::uri;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ChtError {
    TooFewArguments,
    InvalidChtShUri(uri::InvalidUri),
}

impl error::Error for ChtError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ChtError::InvalidChtShUri(ref err) => Some(err),
            ChtError::TooFewArguments => None,
        }
    }
}

impl fmt::Display for ChtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ChtError::InvalidChtShUri(ref _err) => f.write_str("invalid cheat.sh URL provided"),
            ChtError::TooFewArguments => f.write_str(
                "a programming language name and an optional query string were not provided",
            ),
        }
    }
}

impl From<uri::InvalidUri> for ChtError {
    fn from(error: uri::InvalidUri) -> Self {
        ChtError::InvalidChtShUri(error)
    }
}
