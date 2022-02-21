mod help;
mod output;
mod render;
mod version;

use super::debug;
use super::{Command, DonePtr, Result};
use crate::core::Args;
use crate::core::Mode;
use crate::parser;
use crate::request;
use crate::shell::os::OsDirs;
use crate::shell::Shell;
use std::io::Write;

pub struct HttpCommand;

impl<OD: OsDirs, O: Write, E: Write> Command<OD, O, E> for HttpCommand {
    fn execute(&self, shell: &mut Shell<OD, O, E>, args: &mut Args, _: DonePtr) -> Result<()> {
        let ws = parser::execute(args)?;
        match ws.mode() {
            Mode::Help => help::show(),
            Mode::Version => version::show(),
            Mode::Debug => debug::show(),
            Mode::Run => {
                {
                    let mut headers = ws.headers.borrow_mut();
                    request::headers::upgrade(&ws, &mut headers);
                }
                let headers = ws.headers.borrow();
                let req_number = 0u8;
                let response = request::execute(&ws, req_number, &headers)?;
                output::render(shell, &ws, req_number, response)?;
            }
        }
        Ok(())
    }
}
