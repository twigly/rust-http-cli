use crate::items::Items;
use crate::request::Method;
use crate::theme::Theme;
use std::cell::RefCell;

use super::{Flags, HeaderMap, Mode};

#[cfg_attr(test, derive(Debug))]
pub struct Workspace {
    pub method: Method,
    pub urls: Vec<String>,
    pub output_redirected: bool,
    pub terminal_columns: u16,
    pub theme: Box<dyn Theme>, // FIXME Create a crate for theme
    pub flags: Flags,
    pub headers: RefCell<HeaderMap>,
    pub items: RefCell<Items>,
    pub raw: Option<String>,
    pub certificate_authority_file: Option<String>,
}

impl Workspace {
    pub fn mode(&self) -> Mode {
        if self.flags.show_help || self.flags.show_short_help {
            Mode::Help
        } else if self.flags.show_version {
            Mode::Version
        } else if self.flags.debug {
            Mode::Debug
        } else {
            Mode::Run
        }
    }
}

impl super::WorkspaceData for Workspace {
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

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    mod workspace {
        use crate::{
            core::{Flags, HeaderMap, Mode, PushDataItem, Workspace, WorkspaceData},
            items::Items,
            request::Method,
            theme::default::DefaultTheme,
        };
        use std::cell::RefCell;

        #[test]
        fn json_flag() {
            let args = Workspace {
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
                certificate_authority_file: None,
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
            let args = Workspace {
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
                certificate_authority_file: None,
            };
            assert_eq!(args.is_json(), true);
            assert_eq!(args.has_items(), true);
            assert_eq!(args.is_form(), false);
            assert_eq!(args.mode(), Mode::Run);
        }

        #[test]
        fn form_flag() {
            let args = Workspace {
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
                certificate_authority_file: None,
            };
            assert_eq!(args.is_json(), false);
            assert_eq!(args.has_items(), false);
            assert_eq!(args.is_form(), true);
            assert_eq!(args.mode(), Mode::Run);
        }

        #[test]
        fn version() {
            let args = Workspace {
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
                certificate_authority_file: None,
            };
            assert_eq!(args.mode(), Mode::Version);
        }

        #[test]
        fn help() {
            let args = Workspace {
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
                certificate_authority_file: None,
            };
            assert_eq!(args.mode(), Mode::Help);
        }
    }
}
