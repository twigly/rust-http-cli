use super::Error;
use crate::terminal::{stream, Terminal};
use std::fmt;

pub fn show(err: &Error) {
    let output_redirected = !stream::is_stdout();
    let res = Terminal::new(!output_redirected).error_with_message(err);
    match res {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoArgs => write!(
                f,
                "try '{} --help' for more information.",
                env!("CARGO_PKG_NAME")
            ),
            Error::MissingUrl => write!(f, "no URL specified."),
            Error::ItemsAndRawMix => write!(f, "not possible to mix raw data and key=value items."),
            Error::TooManyRaw => write!(f, "only one raw data item is allowed."),
            Error::ContradictoryScheme => write!(f, "either http or https."),
            Error::Unexpected(err) => write!(
                f,
                "found argument '{}' which wasn't expected, or isn't valid in this context.",
                err
            ),
            Error::InvalidFlag(args) => write!(
                f,
                "found argument '{}' which wasn't expected, or isn't valid in this context.",
                args
            ),
            Error::InvalidHeader(err) => write!(f, "invalid header '{}'.", err),
            Error::InvalidItem(err) => write!(f, "invalid item '{}'.", err),
            Error::BadHeaderName(_) => write!(f, "invalid header name."),
            Error::BadHeaderValue(_) => write!(f, "invalid header value."),
            Error::Request(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
            Error::Terminal => write!(f, "can't print in the terminal"),
        }
    }
}
