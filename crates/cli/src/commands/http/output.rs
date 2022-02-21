use super::render::RequestRender;
use super::render::ResponseRender;
use crate::core::Result;
use crate::core::Workspace;
use crate::request::Response;
use crate::shell::os::OsDirs;
use crate::shell::Shell;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::io::Read;
use std::io::Write;

pub fn render<OD: OsDirs, O: Write, E: Write>(shell: &mut Shell<OD, O, E>, ws: &Workspace, _req_number: u8, response: Response) -> Result<()> {
    if ws.output_redirected && !ws.flags.borrow().use_color {
        render_raw_content(ws, RefCell::new(response))?;
    } else {
        let style_enabled = shell.enable_colors();

        let headers = ws.headers.borrow();
        let rf = RequestRender::new(ws, &headers, ws.theme.as_ref(), style_enabled);
        shell.out(rf)?;

        let rf = ResponseRender::new(ws, RefCell::new(response), ws.theme.as_ref(), style_enabled);
        shell.out(rf)?;
    }
    Ok(())
}

fn render_raw_content(_args: &Workspace, response: RefCell<Response>) -> io::Result<()> {
    let mut bytes = Vec::new();
    let mut response = response.borrow_mut();
    response.read_to_end(&mut bytes)?;
    io::stdout().write_all(&bytes)
}
