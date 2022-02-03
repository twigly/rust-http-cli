mod body;
pub(crate) mod header;
pub(crate) mod headers;

use crate::core::{Args, Error, Result};
use body::Body;
use reqwest::{
    blocking::{self, RequestBuilder},
    header::HeaderMap,
};

pub type Response = reqwest::blocking::Response;
pub type Method = reqwest::Method;

pub trait Request {
    fn request(&self, method: Method, url: String) -> RequestBuilder;
}

pub fn execute(args: &Args, req_number: u8, headers: &HeaderMap) -> Result<Response> {
    let client = blocking::Client::new();
    execute_request(&args, req_number, headers, client)
}

fn execute_request(
    args: &Args,
    req_number: u8,
    headers: &HeaderMap,
    builder: blocking::Client,
) -> Result<Response> {
    let method = args.method.clone();
    let url = &args.urls[req_number as usize];

    let response = builder
        .request(method, url)
        .headers(headers.clone())
        .body_if_items(&args)
        .send()?;

    Ok(response)
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Request(err.to_string())
    }
}
