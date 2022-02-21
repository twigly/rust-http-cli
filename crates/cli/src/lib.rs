pub mod app;
mod commands;
mod core;
mod items;
mod macros;
mod parser;
mod request;
pub mod shell;
#[cfg(test)]
pub mod test;
mod theme;

use crate::app::App;
use crate::shell::os::OsDirs;
use crate::shell::Shell;
use std::io::Write;

#[inline]
pub fn run<'a, OD: OsDirs, O: Write, E: Write>(args: &mut Vec<String>, shell: &'a mut Shell<'a, OD, O, E>) -> i32 {
    let mut app = App::new(shell);
    match app.run(args) {
        Ok(_) => app.exit_code(None),
        Err(err) => app.exit_code(Some(err)),
    }
}
