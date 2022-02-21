mod body;
mod certificate;

pub(crate) mod header;
pub(crate) mod headers;
use crate::core::{Error, Result, Workspace};
use body::Body;
use std::time::Duration;

pub type Response = reqwest::blocking::Response;
pub type Method = reqwest::Method;
pub type HeaderMap = reqwest::header::HeaderMap;

pub fn execute(args: &Workspace, req_number: u8, headers: &HeaderMap) -> Result<Response> {
    let mut client_builder = reqwest::blocking::Client::builder()
        .default_headers(headers.clone())
        .gzip(false)
        .timeout(Duration::from_secs(10));

    if let Some(cafile) = args.certificate_authority_file.as_ref() {
        let cert = certificate::load(cafile)?;
        client_builder = client_builder.add_root_certificate(cert);
    }

    let client = client_builder.build()?;
    let method = args.method.clone();
    let url = &args.urls[req_number as usize];
    let response = client.request(method, url).body_if_items(args).send()?;
    Ok(response)
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Request(err.to_string())
    }
}
