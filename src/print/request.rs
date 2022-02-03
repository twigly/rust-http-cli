use super::direction;
use super::HasRendered;
use crate::core::{ArgItems, Args};
use crate::terminal;
use crate::theme::style::Style;

pub fn render_method_and_url(args: &Args, req_number: u8) -> HasRendered {
    let flags = &args.flags;
    if flags.show_request_url {
        let style = &args.theme.request().url();
        direction::render(&args, true);
        terminal::print_and_space(&style, args.method.as_str());
        render_url(&args.urls, req_number as usize, style);
        return HasRendered::Something;
    }
    HasRendered::Nothing
}

pub fn render_body(args: &Args) -> HasRendered {
    let flags = &args.flags;
    if flags.show_request_body {
        if args.has_items() {
            let items = args.items.borrow();
            if args.is_json() {
                terminal::json::println(&items, flags.show_request_compact);
            } else {
                terminal::form::println(&items, flags.show_request_compact);
            }
            return HasRendered::Something;
        } else if let Some(ref raw) = args.raw {
            println!("{}", raw);
            return HasRendered::Something;
        }
    }
    HasRendered::Nothing
}

fn render_url(urls: &[String], req_number: usize, style: &Style) {
    if urls.len() > req_number {
        terminal::println(&style, &urls[req_number]);
    } else {
        terminal::println(&style, "??");
    }
}
