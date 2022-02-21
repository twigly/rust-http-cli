use crate::commands::alias::COMMAND_ALIAS;
use crate::commands::ALIAS_NAME_PREFIX;
use crate::core::Error as CoreError;
use crate::rh_name;
use std::fmt;
use std::io;
use std::io::ErrorKind as IoErrorKind;

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum Error {
    CannotCreateAlias(String, ErrorKind),
    CannotLoadAlias(String, ErrorKind),
    CannotDeleteAlias(String, ErrorKind),
    CannotListAlias,
    Io(ErrorKind),
    NoArgs,
    MissingArgsForAdd,
    TooManyArgsForDelete(String),
    TooManyArgsForList,
}

impl From<Error> for CoreError {
    fn from(err: Error) -> CoreError {
        CoreError::AliasCommand(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err.kind().into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CannotCreateAlias(alias_name, err) => {
                write!(f, "{} (alias name {}{})", err.as_str(), ALIAS_NAME_PREFIX, alias_name)
            }
            Error::CannotLoadAlias(alias_name, err) => {
                write!(f, "{} (alias name {}{})", err.as_str(), ALIAS_NAME_PREFIX, alias_name)
            }
            Error::CannotDeleteAlias(alias_name, err) => {
                write!(f, "{} (alias name {}{})", err.as_str(), ALIAS_NAME_PREFIX, alias_name)
            }
            Error::CannotListAlias => write!(f, "cannot list aliases"),
            Error::Io(err) => write!(f, "{}", err.as_str()),
            Error::NoArgs => write!(f, "missing arguments"),
            Error::MissingArgsForAdd => write!(f, "missing arguments for the --add command"),
            Error::TooManyArgsForDelete(first_arg) => {
                write!(
                    f,
                    "too many arguments for the --delete command\ndid you mean: {} {} --delete {}{}",
                    rh_name!(),
                    COMMAND_ALIAS,
                    ALIAS_NAME_PREFIX,
                    first_arg
                )
            }
            Error::TooManyArgsForList => {
                write!(f, "too many arguments for the --list command")
            }
        }
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum ErrorKind {
    ConfigDirectoryNotFound,
    InvalidConfigDirectory,
    CannotCreateAppConfigDirectory,
    AliasFileNotFound,
    AliasFilePermissionDenied,
    Unknown,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            ConfigDirectoryNotFound => "config directory not found",
            InvalidConfigDirectory => "invalid config directory",
            CannotCreateAppConfigDirectory => "cannot create the app config directory",
            AliasFileNotFound => "alias not found",
            AliasFilePermissionDenied => "permission denied",
            Unknown => "unknown alias error",
        }
    }
}

impl From<IoErrorKind> for ErrorKind {
    fn from(err: IoErrorKind) -> ErrorKind {
        match err {
            IoErrorKind::NotFound => ErrorKind::AliasFileNotFound,
            IoErrorKind::PermissionDenied => ErrorKind::AliasFilePermissionDenied,
            _ => ErrorKind::Unknown,
        }
    }
}
