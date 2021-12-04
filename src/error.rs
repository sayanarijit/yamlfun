use crate::yaml::Error as YamlError;
use crate::{Expr, Value, Yaml};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0} is not defined")]
    Undefined(String),

    #[error("YamlError")]
    YamlError(#[from] YamlError),

    #[error("there is no function to call")]
    NoFunction,

    #[error("{0} is nor a function")]
    NotAFunction(Value),

    #[error("{0} is not a boolean")]
    NotABoolean(Value),

    #[error("{0:?} is not a field")]
    NotAField(Yaml),

    #[error("{0} is not a record")]
    NotARecord(Value),

    #[error("{0} is not a record")]
    NotARecordExpr(Expr),

    #[error("{0} was called with invalid arguments: {1:?}")]
    InvalidArguments(String, Vec<Value>),

    #[error("{0} requires at-least {1} arguments, but {2} was provided")]
    NotEnoughArguments(String, usize, usize),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
    //
    #[error("{0}")]
    PlatformCallError(String),

}

pub type Result<T> = std::result::Result<T, Error>;
