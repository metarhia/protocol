use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum NapiError {
    InvalidArg(String),
    ObjectExpected(String),
    StringExpected(String),
    NameExpected(String),
    FunctionExpected(String),
    NumberExpected(String),
    BooleanExpected(String),
    ArrayExpected(String),
    GenericFailure(String),
    PendingException(String),
    Cancelled(String),
    EscapeCalledTwice(String),
    UnknownError(String),
}

impl Error for NapiError {
    fn description(&self) -> &str {
        match *self {
            NapiError::InvalidArg(_) => "NapiError: invalid argument",
            NapiError::ObjectExpected(_) => "NapiError: object expected",
            NapiError::StringExpected(_) => "NapiError: string expected",
            NapiError::NameExpected(_) => "NapiError: name expected",
            NapiError::FunctionExpected(_) => "NapiError: function expected",
            NapiError::NumberExpected(_) => "NapiError: number expected",
            NapiError::BooleanExpected(_) => "NapiError: boolean argument",
            NapiError::ArrayExpected(_) => "NapiError: array expected",
            NapiError::GenericFailure(_) => "NapiError: generic failure",
            NapiError::PendingException(_) => "NapiError: pending exception",
            NapiError::Cancelled(_) => "NapiError: cancelled",
            NapiError::EscapeCalledTwice(_) => "NapiError: escape called twice",
            NapiError::UnknownError(_) => "NapiError: unknown error",
        }
    }
}

impl Display for NapiError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} ", self.description())?;

        match *self {
            NapiError::InvalidArg(ref message) |
            NapiError::ObjectExpected(ref message) |
            NapiError::StringExpected(ref message) |
            NapiError::NameExpected(ref message) |
            NapiError::FunctionExpected(ref message) |
            NapiError::NumberExpected(ref message) |
            NapiError::BooleanExpected(ref message) |
            NapiError::ArrayExpected(ref message) |
            NapiError::GenericFailure(ref message) |
            NapiError::PendingException(ref message) |
            NapiError::Cancelled(ref message) |
            NapiError::EscapeCalledTwice(ref message) |
            NapiError::UnknownError(ref message) => {
                write!(formatter, "({})", message)
            }
        }
    }
}

pub type NapiResult<T> = Result<T, NapiError>;
