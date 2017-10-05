use std::error::Error;
use std::fmt;
use std::fmt::Display;

use sys::{napi_status, napi_value};

#[derive(Clone, Debug)]
pub struct NapiErrorData {
    pub message: String,
    pub exception: Option<napi_value>,
}

#[derive(Clone, Debug)]
pub enum NapiError {
    InvalidArg(NapiErrorData),
    ObjectExpected(NapiErrorData),
    StringExpected(NapiErrorData),
    NameExpected(NapiErrorData),
    FunctionExpected(NapiErrorData),
    NumberExpected(NapiErrorData),
    BooleanExpected(NapiErrorData),
    ArrayExpected(NapiErrorData),
    GenericFailure(NapiErrorData),
    PendingException(NapiErrorData),
    Cancelled(NapiErrorData),
    EscapeCalledTwice(NapiErrorData),
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
            NapiError::InvalidArg(ref payload) |
            NapiError::ObjectExpected(ref payload) |
            NapiError::StringExpected(ref payload) |
            NapiError::NameExpected(ref payload) |
            NapiError::FunctionExpected(ref payload) |
            NapiError::NumberExpected(ref payload) |
            NapiError::BooleanExpected(ref payload) |
            NapiError::ArrayExpected(ref payload) |
            NapiError::GenericFailure(ref payload) |
            NapiError::PendingException(ref payload) |
            NapiError::Cancelled(ref payload) |
            NapiError::EscapeCalledTwice(ref payload) => {
                write!(formatter, "({})", &payload.message).and_then(
                    |result| {
                        if payload.exception.is_some() {
                            write!(formatter, ", JavaScript exception attached")
                        } else {
                            Ok(result)
                        }
                    },
                )
            }
        }
    }
}

impl NapiError {
    pub fn new(
        status: napi_status,
        message: String,
        exception: Option<napi_value>,
    ) -> Self {
        let payload = NapiErrorData { message, exception };

        match status {
            napi_status::napi_invalid_arg => NapiError::InvalidArg(payload),
            napi_status::napi_object_expected => NapiError::ObjectExpected(
                payload,
            ),
            napi_status::napi_string_expected => NapiError::StringExpected(
                payload,
            ),
            napi_status::napi_name_expected => NapiError::NameExpected(payload),
            napi_status::napi_function_expected => {
                NapiError::FunctionExpected(payload)
            }
            napi_status::napi_number_expected => NapiError::NumberExpected(
                payload,
            ),
            napi_status::napi_boolean_expected => NapiError::BooleanExpected(
                payload,
            ),
            napi_status::napi_array_expected => NapiError::ArrayExpected(
                payload,
            ),
            napi_status::napi_generic_failure => NapiError::GenericFailure(
                payload,
            ),
            napi_status::napi_pending_exception => {
                NapiError::PendingException(payload)
            }
            napi_status::napi_cancelled => NapiError::Cancelled(payload),
            napi_status::napi_escape_called_twice => {
                NapiError::EscapeCalledTwice(payload)
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
