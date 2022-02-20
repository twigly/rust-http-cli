#[cfg(feature = "spinner")]
mod busy;
mod error;
mod exit;

use crate::commands::ArgsCommand;
use crate::core::{Args, Error, Result};
use crate::shell::os::OsDirs;
use crate::shell::Shell;
use std::io::Write;

#[cfg(feature = "spinner")]
use busy::Spinner;

pub struct App<'a, OD, O, E> {
    shell: &'a mut Shell<'a, OD, O, E>,
    #[cfg(feature = "spinner")]
    busy: Spinner,
}

impl<'a, OD: OsDirs, O: Write, E: Write> App<'a, OD, O, E> {
    pub fn new(shell: &'a mut Shell<'a, OD, O, E>) -> Self {
        Self {
            shell,
            #[cfg(feature = "spinner")]
            busy: busy::Spinner::new(),
        }
    }

    pub fn exit(self, err: Option<Error>) {
        #[cfg(feature = "spinner")]
        self.busy.done();

        match err {
            Some(err) => {
                error::show(self.shell, &err);
                exit::error(err);
            }
            None => exit::success(),
        }
    }

    pub fn run(&mut self, args: &mut Args) -> Result<()> {
        let command = args.command(self.shell.os_dirs())?;
        command.execute(self.shell, args, || {})?;

        #[cfg(feature = "spinner")]
        self.busy.clone().done();

        // self.shell.flush()?;
        Ok(())
    }
}
