use reqwest::Certificate;

use crate::core::Result;
use std::io::Read;
use std::{fs::File, path::Path};

pub fn load<P: AsRef<Path>>(path: P) -> Result<Certificate> {
    let mut buf = Vec::new();
    File::open(&path)?.read_to_end(&mut buf)?;
    let cert = certificate(path, &buf)?;
    Ok(cert)
}

fn certificate<P: AsRef<Path>>(path: P, buf: &[u8]) -> Result<Certificate> {
    let cert = if Some(std::ffi::OsStr::new("der")) == path.as_ref().extension() {
        Certificate::from_der(buf)
    } else {
        Certificate::from_pem(buf)
    }?;
    Ok(cert)
}
