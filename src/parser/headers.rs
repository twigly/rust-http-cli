pub use crate::core::HeaderMap;
use crate::core::{Error, PushDataItem, Result};
use reqwest::header::{HeaderName, HeaderValue};
use std::str::FromStr;

impl PushDataItem for HeaderMap {
    fn push(&mut self, item: &str) -> Result<()> {
        match item.split_once(":") {
            Some(parts) => {
                let key = HeaderName::from_str(parts.0)?;
                let value = HeaderValue::from_str(parts.1)?;
                self.append(key, value);
            }
            None => return Err(Error::InvalidHeader(item.into())),
        };
        Ok(())
    }
}

impl From<reqwest::header::InvalidHeaderName> for Error {
    fn from(err: reqwest::header::InvalidHeaderName) -> Error {
        Error::BadHeaderName(err.to_string()) // FIXME The err doesn't contain the header name
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Error {
        Error::BadHeaderValue(err.to_string()) // FIXME The err doesn't contain the header value
    }
}
