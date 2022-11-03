use std::io;
use serde_yaml;

#[derive(Debug)]
enum ErrorRepr {
    IoError(io::Error),
    SerdeError(serde_yaml::Error),
}

#[derive(Debug)]
pub struct TrunError {
    repr: ErrorRepr,
}

/// Library generic result type.
pub type TrunResult<T> = Result<T, TrunError>;


impl From<io::Error> for TrunError {
    fn from(err: io::Error) -> TrunError {
        TrunError { repr: ErrorRepr::IoError(err) }
    }
}

impl From<serde_yaml::Error> for TrunError {
    fn from(err: serde_yaml::Error) -> TrunError {
        TrunError { repr: ErrorRepr::SerdeError(err) }
    }
}
