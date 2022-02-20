use super::Error;
use crate::rh_name;
use crate::shell::os::OsDirs;
use crate::shell::{error::ErrorRender, Shell};
use std::fmt;
use std::io::Write;

pub fn show<OD: OsDirs, O: Write, E: Write>(shell: &mut Shell<OD, O, E>, err: &Error) {
    let rf = ErrorRender::new(err);
    let res = shell.err(rf);
    match res {
        Ok(_) => {}
        Err(err) => {
            let _ = writeln!(std::io::stderr(), "{}", err);
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoArgs => write!(f, "try '{} --help' for more information.", rh_name!()),
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
            #[cfg(feature = "alias")]
            Error::AliasCommand(err) => {
                writeln!(f, "the alias subcommand failed, {}", err)?;
                write!(
                    f,
                    "try '{} {} --help' for more information.",
                    rh_name!(),
                    crate::commands::alias::COMMAND_ALIAS
                )
            }
            #[cfg(feature = "alias")]
            Error::Alias(err) => {
                writeln!(f, "cannot find the alias '{}'", err)?;
                write!(
                    f,
                    "try '{} {} --help' for more information.",
                    rh_name!(),
                    crate::commands::alias::COMMAND_ALIAS
                )
            }
            #[cfg(feature = "alias")]
            Error::AliasOther => {  // FIXME To be removed
                writeln!(f, "unknown error with alias")?;
                write!(
                    f,
                    "try '{} {} --help' for more information.",
                    rh_name!(),
                    crate::commands::alias::COMMAND_ALIAS
                )
            }
        }
    }
}
