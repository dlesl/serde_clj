use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    JNI(jni::errors::Error),
    DeserializeAnyNotSupported,
    ExpectedBoolean,
    ExpectedBytes,
    ExpectedInteger,
    ExpectedFloat,
    ExpectedChar,
    ExpectedString,
    ExpectedKeyword,
    ExpectedNull,
    ExpectedArray,
    ExpectedMap,
    ExpectedEnum,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Message(ref msg) => formatter.write_str(msg),
            Error::JNI(ref error) => write!(formatter, "JNI error: {}", error),
            Error::DeserializeAnyNotSupported => formatter.write_str("deserialize_any not supported!"),
            Error::ExpectedBytes => formatter.write_str("ExpectedBytes"),
            Error::ExpectedBoolean => formatter.write_str("ExpectedBoolean"),
            Error::ExpectedInteger => formatter.write_str("ExpectedInteger"),
            Error::ExpectedFloat => formatter.write_str("ExpectedFloat"),
            Error::ExpectedChar => formatter.write_str("ExpectedChar"),
            Error::ExpectedString => formatter.write_str("ExpectedString"),
            Error::ExpectedKeyword => formatter.write_str("ExpectedKeyword"),
            Error::ExpectedNull => formatter.write_str("ExpectedNull"),
            Error::ExpectedArray => formatter.write_str("ExpectedArray"),
            Error::ExpectedMap => formatter.write_str("ExpectedMap"),
            Error::ExpectedEnum => formatter.write_str("ExpectedEnum"),
        }
    }
}

impl std::error::Error for Error {}

impl From<jni::errors::Error> for Error {
    fn from(error: jni::errors::Error) -> Self {
        Error::JNI(error)
    }
}
