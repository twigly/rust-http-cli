#[cfg(feature = "alias")]
use crate::commands::alias::Error as AliasError;

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum Error {
    NoArgs,
    MissingUrl,
    ItemsAndRawMix,
    TooManyRaw,
    ContradictoryScheme,
    Unexpected(String),
    InvalidFlag(String),
    InvalidHeader(String),
    InvalidItem(String),
    BadHeaderName(String),
    BadHeaderValue(String),
    Request(String),
    Io(String),
    #[cfg(feature = "alias")]
    AliasCommand(AliasError),
    #[cfg(feature = "alias")]
    Alias(String),
    #[cfg(feature = "alias")]
    AliasOther, // FIXME To be removed
}
