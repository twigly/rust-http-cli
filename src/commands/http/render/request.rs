use super::{HeaderRender, Render, RequestRender, DIRECTION_REQUEST};
use crate::items::Items;
use crate::request::HeaderMap;
use crate::shell::form::FormRender;
use crate::shell::json::JsonRender;
use crate::{
    core::{Workspace, WorkspaceData},
    theme::Theme,
};
use std::io::{Result, Write};

impl<'a> RequestRender<'a> {
    pub fn new(workspace: &'a Workspace, headers: &'a HeaderMap, theme: &'a dyn Theme, style_enabled: bool) -> Self {
        Self {
            workspace,
            headers,
            theme,
            style_enabled,
            req_number: 0,
        }
    }
}

impl<'a> Render for RequestRender<'a> {
    #[inline]
    fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        let ws = self.workspace;
        let flags = ws.flags;
        if flags.show_request_url {
            if flags.show_direction {
                self.write_direction(writer, true)?;
            }
            self.write_method(writer)?;
            writer.write_all(b" ")?;
            self.write_url(writer)?;
            self.write_newline(writer)?;
        }
        if flags.show_request_headers {
            self.write_headers(writer)?;
        }
        if flags.show_request_body {
            self.write_body(writer)?;
        }
        Ok(())
    }

    #[inline]
    fn is_style_active(&self) -> bool {
        self.style_enabled
    }
}

impl<'a> RequestRender<'a> {
    #[inline]
    fn write_direction<W: Write>(&self, writer: &mut W, is_standard: bool) -> Result<()> {
        self.write_with_style(writer, DIRECTION_REQUEST, &self.theme.request().direction(is_standard))
    }

    #[inline]
    fn write_method<W: Write>(&self, writer: &mut W) -> Result<()> {
        let ws = self.workspace;
        self.write_with_style(writer, ws.method.as_str().as_bytes(), &self.theme.request().method())
    }

    #[inline]
    fn write_url<W: Write>(&self, writer: &mut W) -> Result<()> {
        let ws = self.workspace;
        let urls: &[String] = &ws.urls;

        self.write_with_style(
            writer,
            if urls.len() > self.req_number { &urls[self.req_number] } else { "??" }.as_bytes(),
            &self.theme.request().url(),
        )
    }

    #[inline]
    fn write_body<W: Write>(&self, writer: &mut W) -> Result<()> {
        let ws = self.workspace;
        if ws.has_items() {
            let flags = ws.flags;
            let items = ws.items.borrow();
            if ws.is_json() {
                let json_render = JsonRender::new(&items as &Items, flags.show_request_compact, self.style_enabled);
                json_render.write(writer)?;
            } else {
                let json_render = FormRender::new(&items as &Items, flags.show_request_compact, self.style_enabled);
                json_render.write(writer)?;
            }
            self.write_newline(writer)?;
        } else if let Some(ref raw) = ws.raw {
            writer.write_all(raw.as_bytes())?;
            self.write_newline(writer)?;
        }
        Ok(())
    }

    #[inline]
    fn write_headers<W: Write>(&self, writer: &mut W) -> Result<()> {
        let request_theme = self.theme.request();
        let header_theme = request_theme.as_header();
        let direction_theme = request_theme.as_direction();
        let header_render = HeaderRender::new(self.workspace, self.headers, header_theme, direction_theme, DIRECTION_REQUEST, self.style_enabled);
        header_render.write(writer)
    }
}
