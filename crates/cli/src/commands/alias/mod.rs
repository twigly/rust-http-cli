mod action;
mod error;
mod help;
pub(crate) mod storage;

use super::{Command, DonePtr, ALIAS_NAME_PREFIX};
use crate::core::Args;
use crate::shell::os::OsDirs;
use crate::shell::Shell;
use action::Action;
pub(crate) use error::Error;
use std::io::Write;
pub use storage::load::from_default;
pub use storage::load::from_name;
use storage::DEFAULT_ALIAS_NAME;

pub type Result<T> = std::result::Result<T, Error>;

pub const COMMAND_ALIAS: &str = "alias";

pub struct AliasCommand;

// FIXME must use the shell abstraction instead of println!()
impl<OD: OsDirs, O: Write, E: Write> Command<OD, O, E> for AliasCommand {
    fn execute(&self, shell: &mut Shell<OD, O, E>, args: &mut Args, _: DonePtr) -> super::Result<()> {
        remove_the_first_arg_that_is_the_alias_command(args);
        let subcommand = action::get(args, is_valid_alias, fix_alias_name)?;

        match subcommand {
            Action::Add => {
                let name = alias_name(args)?;
                storage::store::save(shell.os_dirs(), &name, args)?;
                println!("Alias '{}' saved", name);
            }
            Action::Delete => {
                let name = alias_name(args)?;
                storage::store::delete(shell.os_dirs(), &name)?;
                println!("Alias '{}' deleted", name);
            }
            Action::List => {
                let mut aliases = storage::show::list(shell.os_dirs())?;
                aliases.sort_unstable_by(|alias1, alias2| alias1.name.cmp(&alias2.name));
                let alias_count = aliases.len();
                if alias_count > 0 {
                    println!("Found {} {}:", alias_count, if alias_count > 1 { "aliases" } else { "alias" });
                    for alias in aliases {
                        println!("{}{:width$} {}", ALIAS_NAME_PREFIX, alias.name, alias.args.join(" "), width = 12);
                    }
                } else {
                    println!("No aliases found");
                }
            }
            Action::Help => {
                help::show();
            }
        };

        Ok(())
    }
}

fn remove_the_first_arg_that_is_the_alias_command(args: &mut Args) {
    if !args.is_empty() {
        args.remove(0);
    }
}

fn is_valid_alias(arg: Option<&String>) -> bool {
    match arg {
        Some(arg) => arg.starts_with(ALIAS_NAME_PREFIX) && arg.len() > 1,
        None => false,
    }
}

fn fix_alias_name(alias_name: &str) -> String {
    if let Some(alias_name) = alias_name.strip_prefix(ALIAS_NAME_PREFIX) {
        alias_name.to_string()
    } else {
        alias_name.to_string()
    }
}

fn alias_name(args: &mut Args) -> Result<String> {
    if let Some(first) = args.first() {
        if let Some(alias_name) = first.strip_prefix(ALIAS_NAME_PREFIX) {
            let alias_name = alias_name.to_string();
            args.remove(0);
            return Ok(alias_name.to_lowercase());
        }
    }
    Ok(DEFAULT_ALIAS_NAME.to_string())
}

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::alias::error::ErrorKind;
    use crate::core::{Error as CoreError, Result};
    use crate::test::alias::*;
    use crate::test::os::{TestInvalidOsDirs, TestNoOsDirs, TestValidOsDirs};
    use crate::{arg_alias, args, commands::Command};

    mod basic {
        use super::*;

        #[test]
        fn fix_alias() {
            assert_eq!(fix_alias_name("hello"), "hello");
            assert_eq!(fix_alias_name("@hello"), "hello");
            assert_eq!(fix_alias_name(""), "");
            assert_eq!(fix_alias_name("@"), "");
        }

        #[test]
        fn valid_alias() {
            assert_eq!(is_valid_alias(Some(&"hello".to_string())), false);
            assert_eq!(is_valid_alias(Some(&"".to_string())), false);
            assert_eq!(is_valid_alias(Some(&ALIAS_NAME_PREFIX.to_string())), false);
            assert!(is_valid_alias(Some(&format!("{}hello", ALIAS_NAME_PREFIX))));
        }
    }

    mod basic_errors {
        use super::*;

        #[test]
        fn error_if_no_args_for_default_alias() {
            setup();

            let mut args = args!["alias"];
            let os_dirs = TestValidOsDirs::new();
            let mut shell = Shell::new(&os_dirs, Vec::new(), Vec::new());
            let command = AliasCommand {};
            let res = command.execute(&mut shell, &mut args, || {});
            assert!(res.is_err());
        }

        #[test]
        fn error_if_no_args_for_custom_alias() {
            setup();
            let mut args = args!["alias", arg_alias!(CUSTOM_ALIAS_NAME_2)];
            let os_dirs = TestValidOsDirs::new();
            let mut shell = Shell::new(&os_dirs, Vec::new(), Vec::new());
            let command = AliasCommand {};
            let res = command.execute(&mut shell, &mut args, || {});
            assert!(res.is_err());
        }
    }

    mod create {
        use super::*;

        fn test_alias_with<OD: OsDirs>(os_dirs: OD, alias_name: &str, mut args: Args) -> Result<Args> {
            setup();
            assert_eq!(alias_exists(alias_name), false);
            let mut shell = Shell::new(&os_dirs, Vec::new(), Vec::new());
            let command = AliasCommand {};
            command.execute(&mut shell, &mut args, || {})?;
            Ok(args)
        }

        mod default {
            use super::*;

            fn test_default_alias_with<OD: OsDirs>(os_dirs: OD, url: &str) -> Result<Args> {
                test_alias_with(os_dirs, DEFAULT_ALIAS_NAME, args!["alias", url.clone()])
            }
            #[test]
            fn default_alias() {
                let args = test_default_alias_with(TestValidOsDirs::new(), "http://test.com/abc").expect("Cannot execute default_alias");
                assert!(alias_exists(DEFAULT_ALIAS_NAME));
                assert_eq!(args, vec!["http://test.com/abc"]);
            }
            #[test]
            fn error_if_no_config_directory_for_default_alias() {
                let res = test_default_alias_with(TestNoOsDirs::new(), "http://test.com/abc");
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotCreateAlias(DEFAULT_ALIAS_NAME.into(), ErrorKind::ConfigDirectoryNotFound))
                );
            }
            #[test]
            fn error_if_invalid_config_directory_for_default_alias() {
                let res = test_default_alias_with(TestInvalidOsDirs::new(), "http://test.com/abc");
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotCreateAlias(DEFAULT_ALIAS_NAME.into(), ErrorKind::InvalidConfigDirectory))
                );
            }
        }

        mod custom {
            use super::*;

            fn test_custom_alias<OD: OsDirs>(os_dirs: OD, url: &str) -> Result<Args> {
                test_alias_with(os_dirs, CUSTOM_ALIAS_NAME_1, args!["alias", arg_alias!(CUSTOM_ALIAS_NAME_1), url.clone()])
            }

            #[test]
            fn custom_alias() {
                let args = test_custom_alias(TestValidOsDirs::new(), "http://test.com/def").expect("Cannot execute custom_alias");
                assert!(alias_exists(CUSTOM_ALIAS_NAME_1));
                assert_eq!(args, vec!["http://test.com/def"]);
            }
            #[test]
            fn error_if_no_config_directory_for_custom_alias() {
                let res = test_custom_alias(TestNoOsDirs::new(), "http://test.com/abc");
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotCreateAlias(CUSTOM_ALIAS_NAME_1.into(), ErrorKind::ConfigDirectoryNotFound))
                );
            }
            #[test]
            fn error_if_invalid_config_directory_for_custom_alias() {
                let res = test_custom_alias(TestInvalidOsDirs::new(), "http://test.com/abc");
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotCreateAlias(CUSTOM_ALIAS_NAME_1.into(), ErrorKind::InvalidConfigDirectory))
                );
            }
        }
    }

    mod only_single_flag {
        use super::*;

        fn delete_alias_with<OD: OsDirs>(os_dirs: OD, alias_name: &str, mut args: Args) -> Result<Args> {
            setup();
            assert_eq!(alias_exists(alias_name), false);
            create_alias_file(alias_name);
            assert!(alias_exists(alias_name));
            let mut shell = Shell::new(&os_dirs, Vec::new(), Vec::new());
            let command = AliasCommand {};
            command.execute(&mut shell, &mut args, || {})?;
            Ok(args)
        }

        mod list {
            use super::*;

            fn list_alias_with<OD: OsDirs>(os_dirs: OD) -> Result<Args> {
                delete_alias_with(os_dirs, DEFAULT_ALIAS_NAME, args!["alias", "--list"])
            }

            #[test]
            fn list() {
                list_alias_with(TestValidOsDirs::new()).expect("Cannot delete default_alias");
                assert!(alias_exists(DEFAULT_ALIAS_NAME));
            }
            #[test]
            fn error_if_no_config_directory_for_list() {
                let res = list_alias_with(TestNoOsDirs::new());
                assert!(res.is_err());
                assert_eq!(res.unwrap_err(), CoreError::AliasCommand(Error::CannotListAlias));
                assert!(alias_exists(DEFAULT_ALIAS_NAME));
            }
            #[test]
            fn error_if_invalid_config_directory_for_list() {
                let res = list_alias_with(TestInvalidOsDirs::new());
                assert!(res.is_err());
                assert_eq!(res.unwrap_err(), CoreError::AliasCommand(Error::CannotListAlias));
                assert!(alias_exists(DEFAULT_ALIAS_NAME));
            }
        }

        mod delete_default {
            use super::*;

            fn delete_default_alias_with<OD: OsDirs>(os_dirs: OD) -> Result<Args> {
                delete_alias_with(os_dirs, DEFAULT_ALIAS_NAME, args!["alias", "--delete"])
            }

            #[test]
            fn delete_default_alias() {
                delete_default_alias_with(TestValidOsDirs::new()).expect("Cannot delete default_alias");
                assert_eq!(alias_exists(DEFAULT_ALIAS_NAME), false);
            }
            #[test]
            fn error_if_no_config_directory_for_delete_default_alias() {
                let res = delete_default_alias_with(TestNoOsDirs::new());
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotDeleteAlias(DEFAULT_ALIAS_NAME.into(), ErrorKind::ConfigDirectoryNotFound))
                );
                assert!(alias_exists(DEFAULT_ALIAS_NAME));
            }
            #[test]
            fn error_if_invalid_config_directory_for_delete_default_alias() {
                let res = delete_default_alias_with(TestInvalidOsDirs::new());
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotDeleteAlias(DEFAULT_ALIAS_NAME.into(), ErrorKind::InvalidConfigDirectory))
                );
                assert!(alias_exists(DEFAULT_ALIAS_NAME));
            }
        }

        mod delete_custom {
            use super::*;

            fn delete_custom_alias_with<OD: OsDirs>(os_dirs: OD) -> Result<Args> {
                delete_alias_with(os_dirs, CUSTOM_ALIAS_NAME_1, args!["alias", "--delete", arg_alias!(CUSTOM_ALIAS_NAME_1)])
            }

            #[test]
            fn delete_custom_alias() {
                delete_custom_alias_with(TestValidOsDirs::new()).expect("Cannot delete custom_alias");
                assert_eq!(alias_exists(CUSTOM_ALIAS_NAME_1), false);
            }
            #[test]
            fn error_if_no_config_directory_for_delete_custom_alias() {
                let res = delete_custom_alias_with(TestNoOsDirs::new());
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotDeleteAlias(CUSTOM_ALIAS_NAME_1.into(), ErrorKind::ConfigDirectoryNotFound))
                );
                assert!(alias_exists(CUSTOM_ALIAS_NAME_1));
            }
            #[test]
            fn error_if_invalid_config_directory_for_delete_custom_alias() {
                let res = delete_custom_alias_with(TestInvalidOsDirs::new());
                assert!(res.is_err());
                assert_eq!(
                    res.unwrap_err(),
                    CoreError::AliasCommand(Error::CannotDeleteAlias(CUSTOM_ALIAS_NAME_1.into(), ErrorKind::InvalidConfigDirectory))
                );
                assert!(alias_exists(CUSTOM_ALIAS_NAME_1));
            }
        }
    }

    mod error_delete_because_of_args {
        use super::*;

        #[test]
        fn error_delete_default_alias() {
            setup();

            let mut args = args!["alias", "--delete", "aBcD"];
            let os_dirs = TestValidOsDirs::new();
            let mut shell = Shell::new(&os_dirs, Vec::new(), Vec::new());
            let command = AliasCommand {};
            let res = command.execute(&mut shell, &mut args, || {});
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), CoreError::AliasCommand(Error::TooManyArgsForDelete("abcd".into())));
        }

        #[test]
        fn error_delete_custom_alias() {
            setup();

            let mut args = args!["alias", "--delete", arg_alias!(CUSTOM_ALIAS_NAME_1), "def"];
            let os_dirs = TestValidOsDirs::new();
            let mut shell = Shell::new(&os_dirs, Vec::new(), Vec::new());
            let command = AliasCommand {};
            let res = command.execute(&mut shell, &mut args, || {});
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), CoreError::AliasCommand(Error::TooManyArgsForDelete(CUSTOM_ALIAS_NAME_1.into())));
        }
    }
}
