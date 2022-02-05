mod core;
pub(crate) mod form; // FIXME Old version
pub(crate) mod json; // FIXME Old version
pub(crate) mod stream;
mod theme;

use crate::theme::style::{Color, Style};
use std::cell::RefMut;
use std::io::prelude::*;
use std::{cell::RefCell, fmt};
use termcolor::{self, ColorSpec, StandardStream, WriteColor};

pub enum TerminalError {
    Io,
}

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't write in the terminal")
    }
}

pub type Result<T> = std::result::Result<T, TerminalError>;

pub struct Terminal {
    stdout: RefCell<StandardStream>,
    stderr: RefCell<StandardStream>,
}

#[cfg(test)]
impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Terminal").finish()
    }
}

impl Terminal {
    pub fn new(use_color: bool) -> Terminal {
        let color_choice = if use_color {
            termcolor::ColorChoice::AlwaysAnsi
        } else {
            termcolor::ColorChoice::Never
        };

        Terminal {
            stdout: RefCell::new(StandardStream::stdout(color_choice)),
            stderr: RefCell::new(StandardStream::stderr(color_choice)),
        }
    }

    pub fn error_with_message<T: fmt::Display>(&self, message: T) -> Result<()> {
        let mut stderr = self.stderr.borrow_mut();
        let style = Color::Red.bold();
        self.write_with_style(&mut stderr, &style, "error: ")?;
        writeln!(stderr, "{}", message)?;
        Ok(())
    }

    pub fn message_with_style<T: fmt::Display>(&mut self, style: &Style, message: T) -> Result<()> {
        self.write_with_style(&mut self.stdout.borrow_mut(), style, message)
    }

    pub fn message<T: fmt::Display>(&self, message: T, newline: bool) -> Result<()> {
        let mut stdout = self.stdout.borrow_mut();
        write!(stdout, "{}", message)?;
        if newline {
            stdout.write_all(b"\n")?;
        }
        Ok(())
    }

    fn write_with_style<T: fmt::Display>(
        &self,
        stream: &mut RefMut<StandardStream>,
        style: &Style,
        message: T,
    ) -> Result<()> {
        stream.set_color(
            ColorSpec::new()
                .set_bold(style.is_bold)
                .set_fg(style.forecolor.map(From::from)),
        )?;
        write!(stream, "{}", message)?;
        if style.newline {
            stream.write_all(b"\n")?;
        }
        stream.reset()?;
        Ok(())
    }
}

impl From<std::io::Error> for TerminalError {
    fn from(_: std::io::Error) -> TerminalError {
        TerminalError::Io
    }
}
