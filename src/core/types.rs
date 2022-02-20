use super::error::Error;

pub type Args = Vec<String>;
pub type HeaderMap = reqwest::header::HeaderMap;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Mode {
    Run,
    Help,
    Version,
    Debug,
}
