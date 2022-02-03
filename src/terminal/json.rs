use super::theme::DefaultTheme;
use crate::items::Items;
use serde::ser::Serialize;
use serde_json::{
    ser::{CompactFormatter, PrettyFormatter},
    Value,
};
use std::io::Write;
use termcolor::{ColorChoice, StandardStream, WriteColor};
use termcolor_json::Theme as TermTheme;

pub fn println(items: &Items, compact: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    let _ = write(
        &mut stdout.lock(),
        &items,
        &DefaultTheme::default(),
        compact,
    );
    let _ = stdout.write(b"\n");
}

pub fn println_from_str(buffer: &str, compact: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::AlwaysAnsi);
    match serde_json::from_str::<Value>(buffer) {
        Ok(json) => {
            let _ = write(&mut stdout.lock(), &json, &DefaultTheme::default(), compact);
            let _ = stdout.write(b"\n");
        }
        Err(_) => println!("{}", buffer),
    }
}

fn write<W, T>(writer: W, value: &T, theme: &TermTheme, compact: bool) -> serde_json::Result<()>
where
    W: WriteColor,
    T: ?Sized + Serialize,
{
    if compact {
        termcolor_json::to_writer_with_theme_and_formatter(writer, value, theme, CompactFormatter)
    } else {
        termcolor_json::to_writer_with_theme_and_formatter(
            writer,
            value,
            theme,
            PrettyFormatter::new(),
        )
    }
}
