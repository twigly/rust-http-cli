use super::direction;
use super::HasRendered;
use crate::core::Args;
use crate::core::Result;
use crate::request::header::StandardHeader;
use reqwest::header::HeaderMap;

pub fn render_request(args: &Args, map: &HeaderMap, request_dir: bool) -> Result<HasRendered> {
    let flags = &args.flags;
    render(flags.show_request_headers, &args, &map, request_dir)
}

pub fn render_response(args: &Args, map: &HeaderMap, request_dir: bool) -> Result<HasRendered> {
    let flags = &args.flags;
    render(flags.show_response_headers, &args, &map, request_dir)
}

fn render(
    show_headers: bool,
    args: &Args,
    map: &HeaderMap,
    request_dir: bool,
) -> Result<HasRendered> {
    if show_headers {
        let request = args.theme.request();
        let response = args.theme.response();
        let theme = if request_dir {
            request.as_header()
        } else {
            response.as_header()
        };

        let mut has_headers = false;
        for (key, value) in map.iter() {
            let is_standard = key.is_standard();
            let key = key.as_str();

            direction::render_with_standard_option(&args, request_dir, is_standard)?;

            args.terminal()
                .message_with_style(&theme.header_name(is_standard), format!("{}: ", key))?;
            args.terminal().message_with_style(
                &theme.header_value(is_standard),
                value.to_str().unwrap_or("No value"),
            )?;
            has_headers = true;
        }
        return if has_headers {
            Ok(HasRendered::Something)
        } else {
            Ok(HasRendered::Nothing)
        };
    }
    Ok(HasRendered::Nothing)
}
