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

pub type NapiResult<T> = Result<T, NapiError>;
