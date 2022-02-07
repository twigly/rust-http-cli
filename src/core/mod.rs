use crate::items::Items;
use crate::request::Method;
use crate::terminal::Terminal;
use crate::theme::Theme;
use std::cell::{RefCell, RefMut};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Mode {
    Run,
    Help,
    Version,
}

#[cfg_attr(test, derive(Debug))]
pub struct Args {
    pub terminal: RefCell<Terminal>, // FIXME Create a crate for terminal
    pub method: Method,
    pub urls: Vec<String>,
    pub output_redirected: bool,
    pub terminal_columns: u16,
    pub theme: Box<dyn Theme>, // FIXME Create a crate for theme
    pub flags: Flags,
    pub headers: RefCell<HeaderMap>,
    pub items: RefCell<Items>,
    pub raw: Option<String>,
}

impl Args {
    pub fn mode(&self) -> Mode {
        if self.flags.show_help {
            Mode::Help
        } else if self.flags.show_version {
            Mode::Version
        } else {
            Mode::Run
        }
    }

    pub fn terminal(&self) -> RefMut<'_, Terminal> {
        self.terminal.borrow_mut()
    }
}

pub trait ArgItems {
    fn is_json(&self) -> bool;
    fn is_form(&self) -> bool;
    fn has_items(&self) -> bool;
}
impl ArgItems for Args {
    fn is_json(&self) -> bool {
        self.flags.as_json || (!self.flags.as_form && self.has_items())
    }
    fn is_form(&self) -> bool {
        self.flags.as_form
    }
    fn has_items(&self) -> bool {
        self.items.borrow().len() > 0
    }
}

#[cfg_attr(test, derive(Debug))]
#[derive(Clone, Copy)]
pub struct Flags {
    pub show_version: bool,
    pub show_help: bool,

    pub https: bool,
    pub http: bool,
    pub use_color: bool,
    pub show_direction: bool,

    pub as_json: bool,
    pub as_form: bool,

    pub show_request_url: bool,
    pub show_request_headers: bool,
    pub show_request_compact: bool,
    pub show_request_body: bool,

    pub show_response_status: bool,
    pub show_response_headers: bool,
    pub show_response_compact: bool,
    pub show_response_body: bool,
}

pub type HeaderMap = reqwest::header::HeaderMap;

// #[derive(Debug, PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub enum Error {
    NoArgs,
    MissingUrl,
    ItemsAndRawMix,
    TooManyRaw,
    ContradictoryScheme,
    Unexpected(String),
    InvalidFlag(String),
    InvalidHeader(String),
    InvalidItem(String),
    BadHeaderName(String),
    BadHeaderValue(String),
    Request(String),
    Io(String),
    Terminal,
    #[cfg(config)]
    Config(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait PushItem {
    fn push(&mut self, item: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::{ArgItems, Args, Flags, HeaderMap, Method, Mode, PushItem, Terminal};

    mod args {
        use super::{ArgItems, Args, Flags, HeaderMap, Method, Mode, PushItem, Terminal};
        use crate::{items::Items, theme::default::DefaultTheme};
        use std::cell::RefCell;

        #[test]
        fn json_flag() {
            let args = Args {
                terminal: RefCell::new(Terminal::new(false)),
                method: Method::GET,
                urls: Vec::new(),
                output_redirected: false,
                terminal_columns: 100,
                theme: Box::new(DefaultTheme {}),
                flags: Flags {
                    as_json: true,
                    ..Flags::default()
                },
                headers: RefCell::new(HeaderMap::new()),
                items: RefCell::new(Items::new()),
                raw: None,
            };
            assert_eq!(args.is_json(), true);
            assert_eq!(args.has_items(), false);
            assert_eq!(args.is_form(), false);
            assert_eq!(args.mode(), Mode::Run);
        }

        #[test]
        fn json_items() {
            let mut items = Items::new();
            let _ = items.push("key=value");
            let args = Args {
                terminal: RefCell::new(Terminal::new(false)),
                method: Method::GET,
                urls: Vec::new(),
                output_redirected: false,
                terminal_columns: 100,
                theme: Box::new(DefaultTheme {}),
                flags: Flags {
                    as_json: false,
                    ..Flags::default()
                },
                headers: RefCell::new(HeaderMap::new()),
                items: RefCell::new(items),
                raw: None,
            };
            assert_eq!(args.is_json(), true);
            assert_eq!(args.has_items(), true);
            assert_eq!(args.is_form(), false);
            assert_eq!(args.mode(), Mode::Run);
        }

        #[test]
        fn form_flag() {
            let args = Args {
                terminal: RefCell::new(Terminal::new(false)),
                method: Method::GET,
                urls: Vec::new(),
                output_redirected: false,
                terminal_columns: 100,
                theme: Box::new(DefaultTheme {}),
                flags: Flags {
                    as_form: true,
                    ..Flags::default()
                },
                headers: RefCell::new(HeaderMap::new()),
                items: RefCell::new(Items::new()),
                raw: None,
            };
            assert_eq!(args.is_json(), false);
            assert_eq!(args.has_items(), false);
            assert_eq!(args.is_form(), true);
            assert_eq!(args.mode(), Mode::Run);
        }

        #[test]
        fn version() {
            let args = Args {
                terminal: RefCell::new(Terminal::new(false)),
                method: Method::GET,
                urls: Vec::new(),
                output_redirected: false,
                terminal_columns: 100,
                theme: Box::new(DefaultTheme {}),
                flags: Flags {
                    show_version: true,
                    ..Flags::default()
                },
                headers: RefCell::new(HeaderMap::new()),
                items: RefCell::new(Items::new()),
                raw: None,
            };
            assert_eq!(args.mode(), Mode::Version);
        }

        #[test]
        fn help() {
            let args = Args {
                terminal: RefCell::new(Terminal::new(false)),
                method: Method::GET,
                urls: Vec::new(),
                output_redirected: false,
                terminal_columns: 100,
                theme: Box::new(DefaultTheme {}),
                flags: Flags {
                    show_help: true,
                    ..Flags::default()
                },
                headers: RefCell::new(HeaderMap::new()),
                items: RefCell::new(Items::new()),
                raw: None,
            };
            assert_eq!(args.mode(), Mode::Help);
        }
    }
}
