mod core;
mod error;
mod flags;
mod headers;
mod method;
mod normalizer;
mod url;

use crate::core::{Args, Error, Flags, Result};
use crate::items::Items;
use crate::terminal::{Terminal, stream};
use crate::theme::default::DefaultTheme;
use normalizer::Normalizer;
use std::cell::RefCell;
use std::io::{self, Read};

pub fn execute(args: &[String]) -> Result<Args> {
    validate_raw_args(&args)?;

    let output_redirected = !stream::is_stdout();
    let mut normalizer = Normalizer::parse(&args, output_redirected, "http", "localhost")?;
    let method = normalizer.method();
    let flags = normalizer.flags;
    let headers = normalizer.headers;
    let items = normalizer.items;
    let urls = normalizer.urls;
    let mut raw = normalizer.raw.take();

    let input_redirected = !stream::is_stdin();
    if !is_flag_only_command(&flags) {
        validate_processed_urls(&urls)?;
        validate_processed_items(&items, &raw, input_redirected)?;
    }

    if input_redirected {
        extract_input_as_raw_data(&mut raw)?;
    }

    Ok(Args {
        terminal: RefCell::new(Terminal::new(flags.use_color)),
        method,
        urls,
        output_redirected,
        terminal_columns: terminal_columns(),
        theme: Box::new(DefaultTheme::new()),
        flags,
        headers: RefCell::new(headers),
        items: RefCell::new(items),
        raw,
    })
}

fn extract_input_as_raw_data(raw: &mut Option<String>) -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    *raw = Some(buffer);
    Ok(())
}

#[inline]
fn validate_raw_args(args: &[String]) -> Result<()> {
    let count = args.len();
    if count == 0 {
        Err(Error::NoArgs)
    } else {
        Ok(())
    }
}

#[inline]
fn validate_processed_urls(urls: &[String]) -> Result<()> {
    if urls.len() == 0 {
        Err(Error::MissingUrl)
    } else {
        Ok(())
    }
}

#[inline]
fn validate_processed_items(
    items: &Items,
    raw: &Option<String>,
    input_redirected: bool,
) -> Result<()> {
    if (items.len() > 0) as u8 + raw.is_some() as u8 + input_redirected as u8 > 1 {
        Err(Error::ItemsAndRawMix)
    } else {
        Ok(())
    }
}

#[inline]
fn is_flag_only_command(flags: &Flags) -> bool {
    flags.show_version || flags.show_help
}

fn terminal_columns() -> u16 {
    match termsize::get() {
        Some(size) => size.cols,
        None => 100,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod basic {
        use super::*;

        #[test]
        fn show_version() {
            let args = crate::args!["--version"];
            let parser = execute(&args).unwrap();
            assert_eq!(parser.flags.show_version, true);
        }

        #[test]
        fn show_help() {
            let args = crate::args!["--help"];
            let parser = execute(&args).unwrap();
            assert_eq!(parser.flags.show_help, true);
        }
    }

    mod validate {
        use super::*;
        use crate::core::PushItem;

        const NO_STDIN_DATA: bool = false;
        const STDIN_DATA: bool = true;

        #[test]
        fn flag_only_commands() {
            let mut flags = Flags::default();
            flags.show_help = true;
            assert!(is_flag_only_command(&flags));

            let mut flags = Flags::default();
            flags.show_version = true;
            assert!(is_flag_only_command(&flags));
        }

        #[test]
        fn error_if_no_args() {
            let args = crate::args![];
            let parser = validate_raw_args(&args);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::NoArgs);
        }

        #[test]
        fn multi_args() {
            let args = crate::args!["GET", "localhost"];
            let parser = validate_raw_args(&args);
            assert!(parser.is_ok());
        }

        #[test]
        fn error_if_no_urls() {
            let parser = validate_processed_urls(&[]);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::MissingUrl);
        }

        #[test]
        fn one_urls() {
            let urls = crate::args!["test.com"];
            let parser = validate_processed_urls(&urls);
            assert!(parser.is_ok());
        }

        #[test]
        fn raw_data() {
            let items = Items::new();
            let raw_data = Some("hello".into());
            let parser = validate_processed_items(&items, &raw_data, NO_STDIN_DATA);
            assert!(parser.is_ok());
        }

        #[test]
        fn key_value() {
            let mut items = Items::new();
            let _ = items.push("key=value");
            let parser = validate_processed_items(&items, &None, NO_STDIN_DATA);
            assert!(parser.is_ok());
        }

        #[test]
        fn stdin() {
            let items = Items::new();
            let parser = validate_processed_items(&items, &None, STDIN_DATA);
            assert!(parser.is_ok());
        }

        #[test]
        fn error_if_mixed_items() {
            let mut items = Items::new();
            let raw_data = Some("hello".into());
            let parser = validate_processed_items(&items, &raw_data, STDIN_DATA);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::ItemsAndRawMix);

            let _ = items.push("key=value");
            let parser = validate_processed_items(&items, &raw_data, NO_STDIN_DATA);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::ItemsAndRawMix);

            let parser = validate_processed_items(&items, &raw_data, STDIN_DATA);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::ItemsAndRawMix);
        }
    }

    mod urls {
        use super::*;

        #[test]
        fn method_and_url() {
            let args = crate::args!["GET", "localhost"];
            let parser = execute(&args).unwrap();
            assert_eq!(parser.urls.len(), 1);
            crate::assert_str_eq!(parser.urls[0], "http://localhost");
        }

        #[test]
        fn method_and_url_and_flag() {
            let args = crate::args!["GET", "localhost", "--headers"];
            let parser = execute(&args).unwrap();
            assert_eq!(parser.urls.len(), 1);
            crate::assert_str_eq!(parser.urls[0], "http://localhost");
            assert_eq!(parser.flags.show_request_headers, true);
            assert_eq!(parser.flags.show_response_headers, true);
        }

        #[test]
        fn detect_obvious_url() {
            let args = crate::args!["GET", "--url", "http://test.com", "--headers"];
            let parser = execute(&args).unwrap();
            assert_eq!(parser.urls.len(), 1);
            crate::assert_str_eq!(parser.urls[0], "http://test.com");
            assert_eq!(parser.flags.show_request_url, true);
            assert_eq!(parser.flags.show_request_headers, true);
            assert_eq!(parser.flags.show_response_headers, true);
        }

        #[test]
        fn error_if_multi_args_including_method_but_method_at_wrong_place() {
            let args = crate::args!["GET", "--url", "--headers", "https://test.com"];
            let parser = execute(&args).unwrap();
            assert_eq!(parser.urls.len(), 1);
            crate::assert_str_eq!(parser.urls[0], "https://test.com");
            assert_eq!(parser.flags.show_request_url, true);
            assert_eq!(parser.flags.show_request_headers, true);
            assert_eq!(parser.flags.show_response_headers, true);
        }

        #[test]
        fn error_if_one_arg_but_no_url() {
            let args: Vec<String> = crate::args!["--url"];
            let parser = execute(&args);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::MissingUrl);
        }

        #[test]
        fn error_if_multi_args_but_no_url() {
            let args = crate::args!["--url", "--headers"];
            let parser = execute(&args);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::MissingUrl);
        }

        #[test]
        fn error_if_multi_args_including_method_but_no_url() {
            let args = crate::args!["GET", "--url", "--headers"];
            let parser = execute(&args);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::MissingUrl);
        }
    }

    mod raw {
        use super::*;

        #[test]
        fn error_if_raw_data_and_json() {
            let args: Vec<String> = crate::args!["test.com", "--raw=data", "key=value"];
            let parser = execute(&args);
            assert!(parser.is_err());
            assert_eq!(parser.unwrap_err(), Error::ItemsAndRawMix);
        }
    }
}
