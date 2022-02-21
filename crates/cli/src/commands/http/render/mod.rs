mod header;
mod request;
mod response;

use crate::core::Workspace;
use crate::request::{HeaderMap, Response};
use crate::shell::Render;
use crate::theme::{DirectionTheme, HeaderTheme, Theme};
use std::cell::RefCell;

pub const DIRECTION_REQUEST: &[u8] = b"> ";
pub const DIRECTION_RESPONSE: &[u8] = b"< ";

pub struct RequestRender<'a> {
    workspace: &'a Workspace,
    headers: &'a HeaderMap,
    theme: &'a dyn Theme,
    style_enabled: bool,
    req_number: usize,
}

pub struct ResponseRender<'a> {
    workspace: &'a Workspace,
    response: RefCell<Response>,
    theme: &'a dyn Theme,
    style_enabled: bool,
}

pub struct HeaderRender<'a> {
    workspace: &'a Workspace,
    headers: &'a HeaderMap,
    header_theme: &'a dyn HeaderTheme,
    direction_theme: &'a dyn DirectionTheme,
    direction_symbol: &'a [u8],
    style_enabled: bool,
}
