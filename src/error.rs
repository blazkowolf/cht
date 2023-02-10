use hyper::{http, http::uri};
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ChtshError {
    InvalidChtShUri(uri::InvalidUri),
    UnknownCheatSheet,
    InvalidChtRequest(http::Error),
    Error(hyper::Error),
    ResponseInterrupted(io::Error),
}

impl error::Error for ChtshError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ChtshError::InvalidChtShUri(ref err) => Some(err),
            ChtshError::UnknownCheatSheet => None,
            ChtshError::InvalidChtRequest(ref err) => Some(err),
            ChtshError::Error(ref err) => Some(err),
            ChtshError::ResponseInterrupted(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ChtshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ChtshError::InvalidChtShUri(ref _err) => f.write_str("invalid cheat.sh URL provided"),
            ChtshError::UnknownCheatSheet => f.write_str("unknown cheat sheet provided"),
            ChtshError::InvalidChtRequest(ref err) => err.fmt(f),
            ChtshError::Error(ref err) => err.fmt(f),
            ChtshError::ResponseInterrupted(ref _err) => {
                f.write_str("response stream from cheat.sh interrupted")
            }
        }
    }
}

impl From<uri::InvalidUri> for ChtshError {
    fn from(error: uri::InvalidUri) -> Self {
        ChtshError::InvalidChtShUri(error)
    }
}

impl From<http::Error> for ChtshError {
    fn from(error: http::Error) -> Self {
        ChtshError::InvalidChtRequest(error)
    }
}

impl From<hyper::Error> for ChtshError {
    fn from(error: hyper::Error) -> Self {
        ChtshError::Error(error)
    }
}

impl From<io::Error> for ChtshError {
    fn from(value: io::Error) -> Self {
        use io::ErrorKind::Interrupted;

        match value.kind() {
            Interrupted => ChtshError::ResponseInterrupted(value),
            _ => todo!(),
        }
    }
}
