mod iter;

use crate::core::{Error, Result};
use iter::FilterOkTrait;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[cfg(test)]
const DEFAULT_CONFIG_ENV_NAME: &str = "RH_CONFIG_DEFAULT";
#[cfg(not(test))]
const DEFAULT_CONFIG_ENV_NAME: &str = "RH_TEST_CONFIG_DEFAULT";

pub fn from_default() -> Result<Vec<String>> {
    match env::var(DEFAULT_CONFIG_ENV_NAME) {
        Ok(args) => Ok(args.split_whitespace().map(From::from).collect()),
        Err(err) => Err(Error::Config("default".to_string(), err.to_string())),
    }
}

pub fn from_name(name: &str) -> Result<Vec<String>> {
    let path = PathBuf::from(format!("/tmp/{}", name));
    match File::open(&path) {
        Ok(file) => from_reader(name, &file),
        Err(err) => Err(Error::Config(name.to_string(), err.to_string())),
    }
}

fn from_reader<R: io::Read>(name: &str, reader: R) -> Result<Vec<String>> {
    let buffer = BufReader::new(reader);
    match buffer.lines().filter_ok(|arg| !arg.is_empty()).collect() {
        Ok(args) => Ok(args),
        Err(err) => Err(Error::Config(name.into(), err.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::{from_reader, Error};

    #[test]
    fn lines() {
        let args = from_reader("testlines", &b"-cushH\nX-KEY-1:val1\nX-KEY-2:val2"[..]).unwrap();
        assert_eq!(args, vec!["-cushH", "X-KEY-1:val1", "X-KEY-2:val2"]);
    }

    #[test]
    fn empty_lines() {
        let args = from_reader(
            "testlines",
            &b"-cushH\n\n\nX-KEY-1:val1\nX-KEY-2:val2\n\n\n"[..],
        )
        .unwrap();
        assert_eq!(args, vec!["-cushH", "X-KEY-1:val1", "X-KEY-2:val2"]);
    }

    #[test]
    fn error_if_invalid_characters() {
        let res = from_reader("testlines", &b"-cushH\nX-KEY-1:\xFFval1\nX-KEY-2:val2"[..]);
        assert!(res.is_err());
        // assert_eq!(res.unwrap_err(), Error::Config("testlines".into(), "testlines".into()));
    }
}
