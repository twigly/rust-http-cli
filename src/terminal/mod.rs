pub(crate) mod form;
pub(crate) mod json;
mod theme;

use crate::theme::style::{Color, Style};
use std::io::Write;
use termcolor::{Color as TermColor, ColorChoice, ColorSpec, StandardStream, WriteColor};

macro_rules! print_and_something {
    ($style:expr, $buffer:expr, $something:expr) => {
        let mut stdout = stdout(&$style);
        let _ = stdout.write($buffer.as_bytes());
        let _ = stdout.write($something);
        let _ = stdout.reset();
    };
}

pub fn print(style: &Style, buffer: &str) {
    let mut stdout = stdout(&style);
    let _ = stdout.write(buffer.as_bytes());
    let _ = stdout.reset();
}

pub fn print_and_space(style: &Style, buffer: &str) {
    print_and_something!(&style, buffer, b" ");
}

pub fn println(style: &Style, buffer: &str) {
    print_and_something!(&style, buffer, b"\n");
}

fn stdout(style: &Style) -> StandardStream {
    let mut stdout = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    match style.forecolor {
        Some(color) => {
            let _ = stdout.set_color(
                ColorSpec::new()
                    .set_fg(Some(TermColor::from(color)))
                    .set_bold(style.is_bold),
            );
        }
        _ => {}
    }
    stdout
}

impl From<Color> for TermColor {
    fn from(color: Color) -> TermColor {
        match color {
            Color::Black => TermColor::Black,
            Color::Red => TermColor::Red,
            Color::Green => TermColor::Green,
            Color::Yellow => TermColor::Yellow,
            Color::Blue => TermColor::Blue,
            Color::Purple => TermColor::Magenta,
            Color::Cyan => TermColor::Cyan,
            Color::White => TermColor::White,
        }
    }
}
