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
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::JNI(ref error) => error.description(),
            Error::DeserializeAnyNotSupported => "deserialize_any not supported!",
            Error::ExpectedBytes => "ExpectedBytes",
            Error::ExpectedBoolean => "ExpectedBoolean",
            Error::ExpectedInteger => "ExpectedInteger",
            Error::ExpectedFloat => "ExpectedFloat",
            Error::ExpectedChar => "ExpectedChar",
            Error::ExpectedString => "ExpectedString",
            Error::ExpectedKeyword => "ExpectedKeyword",
            Error::ExpectedNull => "ExpectedNull",
            Error::ExpectedArray => "ExpectedArray",
            Error::ExpectedMap => "ExpectedMap",
            Error::ExpectedEnum => "ExpectedEnum",
        }
    }
}

impl From<jni::errors::Error> for Error {
    fn from(error: jni::errors::Error) -> Self {
        Error::JNI(error)
    }
}
