use super::Render;
use crate::theme::style::Color;
use std::{
    fmt::Display,
    io::{Result, Write},
};

pub struct ErrorRender<T> {
    message: T,
}

impl<T: Display> ErrorRender<T> {
    pub fn new(message: T) -> Self {
        Self { message }
    }
}

impl<T: Display> Render for ErrorRender<T> {
    #[inline]
    fn is_style_active(&self) -> bool {
        true
    }

    #[inline]
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.write_with_style(writer, "Error: ".as_bytes(), &Color::Red.bold())?;
        writeln!(writer, "{}", self.message)?;
        Ok(())
    }
}
