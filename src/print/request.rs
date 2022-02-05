use super::direction;
use super::HasRendered;
use crate::core::Result;
use crate::core::{ArgItems, Args};
use crate::terminal;
use crate::theme::style::Style;

pub fn render_method_and_url(args: &Args, req_number: u8) -> Result<HasRendered> {
    let flags = &args.flags;
    if flags.show_request_url {
        direction::render(args, true)?;
        args.terminal().message_with_style(
            &args.theme.request().method(),
            format!("{} ", args.method.as_str()),
        )?;
        render_url(args, req_number as usize, &args.theme.request().url())?;
        return Ok(HasRendered::Something);
    }
    Ok(HasRendered::Nothing)
}

pub fn render_body(args: &Args) -> Result<HasRendered> {
    let flags = &args.flags;
    if flags.show_request_body {
        if args.has_items() {
            let items = args.items.borrow();
            if args.is_json() {
                // FIXME Use the args.terminal() instead of the old terminal::...
                terminal::json::println(&items, flags.show_request_compact);
            } else {
                // FIXME Use the args.terminal() instead of the old terminal::...
                terminal::form::println(&items, flags.show_request_compact);
            }
            return Ok(HasRendered::Something);
        } else if let Some(ref raw) = args.raw {
            println!("{}", raw);
            return Ok(HasRendered::Something);
        }
    }
    Ok(HasRendered::Nothing)
}

fn render_url(args: &Args, req_number: usize, style: &Style) -> Result<()> {
    let urls: &[String] = &args.urls;
    args.terminal().message_with_style(
        style,
        if urls.len() > req_number {
            &urls[req_number]
        } else {
            "??"
        },
    )?;
    Ok(())
}
