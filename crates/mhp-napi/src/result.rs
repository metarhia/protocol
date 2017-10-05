use std::error::Error;
use std::fmt;
use std::fmt::Display;

use sys::napi_status;

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
            NapiError::EscapeCalledTwice(ref message) => {
                write!(formatter, "({})", message)
            }
        }
    }
}

impl NapiError {
    pub fn new(status: napi_status, message: String) -> Self {
        match status {
            napi_status::napi_invalid_arg => NapiError::InvalidArg(message),
            napi_status::napi_object_expected => NapiError::ObjectExpected(
                message,
            ),
            napi_status::napi_string_expected => NapiError::StringExpected(
                message,
            ),
            napi_status::napi_name_expected => NapiError::NameExpected(message),
            napi_status::napi_function_expected => {
                NapiError::FunctionExpected(message)
            }
            napi_status::napi_number_expected => NapiError::NumberExpected(
                message,
            ),
            napi_status::napi_boolean_expected => NapiError::BooleanExpected(
                message,
            ),
            napi_status::napi_array_expected => NapiError::ArrayExpected(
                message,
            ),
            napi_status::napi_generic_failure => NapiError::GenericFailure(
                message,
            ),
            napi_status::napi_pending_exception => {
                NapiError::PendingException(message)
            }
            napi_status::napi_cancelled => NapiError::Cancelled(message),
            napi_status::napi_escape_called_twice => {
                NapiError::EscapeCalledTwice(message)
            }
            _ => {
                // Both situations should never happen, so just panic.
                panic!(
                    "Either the JavaScript VM returned an unknown status code, \
                    or NapiError::new was called with napi_status::napi_ok"
                );
            }
        }
    }
}

pub type NapiResult<T> = Result<T, NapiError>;
