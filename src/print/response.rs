use super::direction;
use super::HasRendered;
use crate::core::Args;
use crate::request::Response;
use crate::terminal;
use content_inspector::inspect;
use std::borrow::Borrow;
use std::io::Read;

pub fn render_status(args: &Args, response: &Response) -> HasRendered {
    let flags = &args.flags;
    if flags.show_response_status {
        let status = response.status();
        let style = &args.theme.response().status();
        direction::render(&args, false);
        terminal::print_and_space(&style, &format!("{:?}", response.version())); // FIXME format!
        terminal::print_and_space(&style, status.as_str());
        terminal::println(&style, status.canonical_reason().unwrap_or("Unknown"));
        return HasRendered::Something;
    }
    HasRendered::Nothing
}

pub fn render_body(args: &Args, response: &mut Response) {
    let flags = &args.flags;
    if flags.show_response_body {
        let mut bytes = Vec::new();
        let size = response.read_to_end(&mut bytes).unwrap_or(0);
        let content_type = inspect(&bytes);
        if content_type.is_binary() {
            print_binary_usage(&args, size);
        } else {
            print_bytes(&bytes, flags.show_response_compact);
        }
    }
}

fn print_bytes(bytes: &[u8], compact: bool) {
    let text = String::from_utf8_lossy(&bytes);
    terminal::json::println_from_str(text.borrow(), compact);
}

fn print_binary_usage(args: &Args, size: usize) {
    println!("Binary data not shown in terminal");
    println!("Content size {}b", size);
    println!("To copy the content in a file, you should try:");
    println!("{} > filename", args.user_command());
}
