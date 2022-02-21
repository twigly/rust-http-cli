use super::{HeaderRender, Render};
use crate::request::header::StandardHeader;
use crate::request::HeaderMap;
use crate::theme::HeaderTheme;
use crate::{core::Workspace, theme::DirectionTheme};
use std::io::{Result, Write};

impl<'a> HeaderRender<'a> {
    pub fn new(
        workspace: &'a Workspace,
        headers: &'a HeaderMap,
        header_theme: &'a dyn HeaderTheme,
        direction_theme: &'a dyn DirectionTheme,
        direction_symbol: &'a [u8],
        style_enabled: bool,
    ) -> Self {
        Self {
            workspace,
            headers,
            header_theme,
            direction_theme,
            direction_symbol,
            style_enabled,
        }
    }
}

impl<'a> Render for HeaderRender<'a> {
    #[inline]
    fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        let flags = self.workspace.flags;
        let header_theme = self.header_theme;

        for (key, value) in self.headers.iter() {
            let is_standard = key.is_standard();
            let key_style = header_theme.header_name(is_standard);
            let key = key.as_str();

            if flags.show_direction {
                self.write_direction(writer, is_standard)?;
            }
            self.write_with_style(writer, key.as_bytes(), &key_style)?;
            self.write_with_style(writer, ": ".as_bytes(), &key_style)?;
            self.write_with_style(writer, value.to_str().unwrap_or("No value").as_bytes(), &header_theme.header_value(is_standard))?;
            self.write_newline(writer)?;
        }
        Ok(())
    }

    #[inline]
    fn is_style_active(&self) -> bool {
        self.style_enabled
    }
}

impl<'a> HeaderRender<'a> {
    #[inline]
    fn write_direction<W: Write>(&self, writer: &mut W, is_standard: bool) -> Result<()> {
        self.write_with_style(writer, self.direction_symbol, &self.direction_theme.direction(is_standard))
    }
}
