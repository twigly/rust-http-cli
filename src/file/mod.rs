use crate::core::Args;
use crate::request::Response;
use std::io::Read;
use std::io::{self, Write};

pub fn render(_args: &Args, response: &mut Response) {
    let mut bytes = Vec::new();
    let _ = response.read_to_end(&mut bytes).unwrap_or(0);
    let _ = io::stdout().write_all(&bytes);
}
