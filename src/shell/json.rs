use super::Render;
use ansi_term::{Color as AnsiTermColor, Style};
use colored_json::{ColoredFormatter, CompactFormatter, PrettyFormatter, Styler};
use serde::Serialize;
use serde_json::ser::Formatter;
use std::io::{Result, Write};

pub struct JsonRender<'a, T> {
    value: &'a T,
    compact: bool,
    style_enabled: bool,
}

impl<'a, T: Serialize> JsonRender<'a, T> {
    pub fn new(value: &'a T, compact: bool, style_enabled: bool) -> Self {
        Self {
            value,
            compact,
            style_enabled,
        }
    }
}

impl<'a, T: Serialize> Render for JsonRender<'a, T> {
    #[inline]
    fn is_style_active(&self) -> bool {
        self.style_enabled
    }

    #[inline]
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        if self.compact {
            self.write_with_formatter(writer, CompactFormatter)
        } else {
            self.write_with_formatter(writer, PrettyFormatter::new())
        }
    }
}

impl<'a, T: Serialize> JsonRender<'a, T> {
    #[inline]
    fn write_with_formatter<W: Write, F: Formatter>(&self, writer: W, formatter: F) -> Result<()> {
        let formatter = ColoredFormatter::with_styler(formatter, self.style());
        let mut serializer = serde_json::Serializer::with_formatter(writer, formatter);
        self.value.serialize(&mut serializer)?;
        Ok(())
    }

    #[inline]
    fn style(&self) -> Styler {
        Styler {
            object_brackets: Style::new(),
            array_brackets: Style::new().fg(AnsiTermColor::Red),
            key: Style::new().fg(AnsiTermColor::Blue),
            string_value: Style::new().fg(AnsiTermColor::Green),
            integer_value: Style::new().fg(AnsiTermColor::Purple),
            float_value: Style::new().fg(AnsiTermColor::Purple),
            bool_value: Style::new().fg(AnsiTermColor::Yellow),
            nil_value: Style::new().fg(AnsiTermColor::Cyan),
            string_include_quotation: true,
        }
    }
}
