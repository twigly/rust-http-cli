use super::Render;
use serde::Serialize;
use std::io::{Result, Write};

pub struct FormRender<'a, T> {
    value: &'a T,
    // compact: bool,
    style_enabled: bool,
}

impl<'a, T: Serialize> FormRender<'a, T> {
    pub fn new(value: &'a T, _compact: bool, style_enabled: bool) -> Self {
        Self {
            value,
            // compact,
            style_enabled,
        }
    }
}

impl<'a, T: Serialize> Render for FormRender<'a, T> {
    #[inline]
    fn is_style_active(&self) -> bool {
        self.style_enabled
    }

    #[inline]
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        match serde_urlencoded::to_string(self.value) {
            Ok(buffer) => {
                writer.write_all(buffer.as_bytes())?;
            }
            Err(_) => {
                writer.write_all(b"Can't render the items as URL encoded")?;
            }
        };
        Ok(())
    }
}
