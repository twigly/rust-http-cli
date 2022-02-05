use super::direction;
use super::HasRendered;
use crate::core::Args;
use crate::core::Result;
use crate::request::Response;
use crate::terminal;
use content_inspector::inspect;
use std::borrow::Borrow;
use std::io::Read;

pub fn render_status(args: &Args, response: &Response) -> Result<HasRendered> {
    let flags = &args.flags;
    if flags.show_response_status {
        let status = response.status();
        let style = &args.theme.response().status();
        direction::render(args, false)?;
        let message = format!(
            "{:?} {} {}",
            response.version(),
            status.as_str(),
            status.canonical_reason().unwrap_or("Unknown")
        );
        args.terminal().message_with_style(style, message)?;
        return Ok(HasRendered::Something);
    }
    Ok(HasRendered::Nothing)
}

pub fn render_body(args: &Args, response: &mut Response) -> Result<()> {
    let flags = &args.flags;
    if flags.show_response_body {
        let mut bytes = Vec::new();
        let size = response.read_to_end(&mut bytes).unwrap_or(0);
        let content_type = inspect(&bytes);
        if content_type.is_binary() {
            print_binary_usage(args, size)?;
        } else {
            print_bytes(&bytes, flags.show_response_compact);
        }
    }
    Ok(())
}

fn print_bytes(bytes: &[u8], compact: bool) {
    let text = String::from_utf8_lossy(bytes);
    // FIXME Use the args.terminal() instead of the old terminal::...
    terminal::json::println_from_str(text.borrow(), compact);
}

fn print_binary_usage(args: &Args, size: usize) -> Result<()> {
    let message = format!(
        "Binary data not shown in terminal\nContent size {}b\nTo copy the content in a file, you should try:\n{} > filename", 
        size,
        env!("CARGO_PKG_NAME")
    );
    args.terminal().message(message, true)?;
    Ok(())
}
