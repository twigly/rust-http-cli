pub(crate) mod error;
pub(crate) mod form;
pub(crate) mod json;
pub(crate) mod os;
pub(crate) mod stream;

use self::os::OsDirs;
use crate::theme::style::{Color, Style};
use ansi_term::Color as AnsiTermColor;
use ansi_term::Style as AnsiTermStyle;
use std::io::{Result, Write};

pub struct Shell<'a, OD, O, E> {
    os_dirs: &'a OD,
    out: O,
    err: E,
}

impl<'a, OD: OsDirs, O: Write, E: Write> Shell<'a, OD, O, E> {
    pub fn new(os_dirs: &'a OD, out: O, err: E) -> Self {
        Self { os_dirs, out, err }
    }

    pub fn out<R: Render>(&mut self, render: R) -> Result<()> {
        render.write(&mut self.out)?;
        Ok(())
    }
    pub fn err<R: Render>(&mut self, render: R) -> Result<()> {
        render.write(&mut self.err)?;
        Ok(())
    }

    pub fn os_dirs(&self) -> &OD {
        self.os_dirs
    }

    // pub fn flush(&mut self) -> Result<()> {
    //     self.out.flush()?;
    //     self.err.flush()
    // }
}

pub trait Render {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn is_style_active(&self) -> bool;

    fn write_newline<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn write_with_style<W: Write>(&self, writer: &mut W, buf: &[u8], style: &Style) -> Result<()> {
        if self.is_style_active() {
            AnsiTermStyle {
                is_bold: style.is_bold,
                is_dimmed: style.is_dimmed,
                foreground: to_ansi_term_color(style.forecolor),
                ..AnsiTermStyle::default()
            }
            .paint(buf)
            .write_to(writer)?;
        } else {
            writer.write_all(buf)?;
        }

        Ok(())
    }
}

#[inline]
fn to_ansi_term_color(color: Option<Color>) -> Option<AnsiTermColor> {
    color.map(|color| color.into())
}

impl From<Color> for AnsiTermColor {
    fn from(color: Color) -> AnsiTermColor {
        match color {
            Color::Black => AnsiTermColor::Black,
            Color::Red => AnsiTermColor::Red,
            Color::Green => AnsiTermColor::Green,
            Color::Yellow => AnsiTermColor::Yellow,
            Color::Blue => AnsiTermColor::Blue,
            Color::Purple => AnsiTermColor::Purple,
            Color::Cyan => AnsiTermColor::Cyan,
            Color::White => AnsiTermColor::White,
        }
    }
}
