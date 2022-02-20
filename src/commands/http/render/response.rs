use super::{HeaderRender, Render, ResponseRender, DIRECTION_RESPONSE};
use crate::request::Response;
use crate::rh_name;
use crate::shell::json::JsonRender;
use crate::{core::Workspace, theme::Theme};
use content_inspector::inspect;
use serde_json::Value;
use std::cell::RefCell;
use std::io::Read;
use std::io::{Result, Write};

impl<'a> ResponseRender<'a> {
    pub fn new(workspace: &'a Workspace, response: RefCell<Response>, theme: &'a dyn Theme, style_enabled: bool) -> Self {
        Self {
            workspace,
            response,
            theme,
            style_enabled,
        }
    }
}

impl<'a> Render for ResponseRender<'a> {
    #[inline]
    fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        let ws = self.workspace;
        let flags = ws.flags;
        if flags.show_response_status {
            if flags.show_direction {
                self.write_direction(writer, true)?;
            }
            self.write_version_and_status(writer)?;
        }
        if flags.show_response_headers {
            self.write_headers(writer)?;
        }
        if flags.show_response_body {
            self.write_body(writer)?;
        }
        Ok(())
    }

    #[inline]
    fn is_style_active(&self) -> bool {
        self.style_enabled
    }
}

impl<'a> ResponseRender<'a> {
    #[inline]
    fn write_direction<W: Write>(&self, writer: &mut W, is_standard: bool) -> Result<()> {
        self.write_with_style(writer, DIRECTION_RESPONSE, &self.theme.response().direction(is_standard))
    }

    #[inline]
    fn write_version_and_status<W: Write>(&self, writer: &mut W) -> Result<()> {
        let response = &self.response.borrow();
        let status = response.status();
        let theme = self.workspace.theme.response();

        let style = theme.version();
        let message = format!("{:?} ", response.version(),);
        self.write_with_style(writer, message.as_bytes(), &style)?;

        let style = theme.status();
        self.write_with_style(writer, status.as_str().as_bytes(), &style)?;
        writer.write_all(b" ")?;
        self.write_with_style(writer, status.canonical_reason().unwrap_or("Unknown").as_bytes(), &style)?;
        self.write_newline(writer)
    }

    #[inline]
    fn write_body<W: Write>(&self, writer: &mut W) -> Result<()> {
        let ws = self.workspace;
        let flags = ws.flags;
        let mut response = self.response.borrow_mut();

        let mut bytes = Vec::new();
        let size = response.read_to_end(&mut bytes).unwrap_or(0);
        let content_type = inspect(&bytes);
        if content_type.is_binary() {
            self.write_binary_usage(writer, size)?;
        } else {
            let body = String::from_utf8_lossy(&bytes);
            match serde_json::from_str::<Value>(&body) {
                Ok(json) => {
                    let json_render = JsonRender::new(&json, flags.show_response_compact, self.style_enabled);
                    json_render.write(writer)?;
                }
                Err(_) => {
                    writer.write_all(body.as_bytes())?;
                }
            }
        }
        self.write_newline(writer)
    }

    #[inline]
    fn write_binary_usage<W: Write>(&self, writer: &mut W, size: usize) -> Result<()> {
        let message = format!(
            "Binary data not shown in terminal\nContent size {}b\nTo copy the content in a file, you should try:\n{} > filename",
            size,
            rh_name!()
        );
        writer.write_all(message.as_bytes())?;
        self.write_newline(writer)?;
        Ok(())
    }

    #[inline]
    fn write_headers<W: Write>(&self, writer: &mut W) -> Result<()> {
        let response = self.response.borrow();
        let headers = response.headers();
        let response_theme = self.theme.response();
        let header_theme = response_theme.as_header();
        let direction_theme = response_theme.as_direction();
        let header_render = HeaderRender::new(self.workspace, headers, header_theme, direction_theme, DIRECTION_RESPONSE, self.style_enabled);
        header_render.write(writer)
    }
}
