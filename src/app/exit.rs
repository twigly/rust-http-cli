use super::Error;
use std::process::exit;

pub const SUCCESS: i32 = 0;
pub const ERROR: i32 = 1;

pub fn success() {
    exit(SUCCESS);
}

pub fn error(_: Error) {
    // FIXME Return an exit code based on the error
    exit(ERROR);
}
