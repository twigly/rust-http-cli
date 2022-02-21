#[cfg(feature = "alias")]
use super::{
    alias::{from_default, from_name, AliasCommand, COMMAND_ALIAS},
    ALIAS_NAME_PREFIX,
};
use super::{http::HttpCommand, ArgsCommand, Command, Result};
use crate::{
    core::{Args, Error},
    shell::os::OsDirs,
};
use std::io::Write;

impl<OD: OsDirs, O: Write, E: Write> ArgsCommand<OD, O, E> for Args {
    fn command(&mut self, os_dirs: &OD) -> Result<Box<dyn Command<OD, O, E>>> {
        #[cfg(feature = "alias")]
        match self.first() {
            Some(first) => {
                if first == COMMAND_ALIAS {
                    Ok(Box::new(AliasCommand {}))
                } else if let Some(alias_name) = first.strip_prefix(ALIAS_NAME_PREFIX) {
                    match from_name(os_dirs, alias_name) {
                        Ok(mut config_args) => {
                            self.splice(..1, config_args.drain(..));
                            Ok(Box::new(HttpCommand {}))
                        }
                        Err(super::alias::Error::CannotLoadAlias(alias_name, _kind)) => Err(Error::Alias(format!("{}{}", ALIAS_NAME_PREFIX, alias_name))),
                        Err(_) => Err(Error::AliasOther), // FIXME Not good
                    }
                } else {
                    if let Ok(mut config_args) = from_default(os_dirs) {
                        self.splice(..0, config_args.drain(..));
                    }
                    Ok(Box::new(HttpCommand {}))
                }
            }
            None => Err(Error::NoArgs),
        }
        #[cfg(not(feature = "alias"))]
        {
            if !self.is_empty() {
                Ok(Box::new(HttpCommand {}))
            } else {
                Err(Error::NoArgs)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::alias::storage::DEFAULT_ALIAS_NAME;
    use crate::test::alias::*;
    use crate::{arg_alias, args, test::os::TestValidOsDirs};

    #[test]
    fn default_alias() {
        setup();
        assert_eq!(alias_exists(DEFAULT_ALIAS_NAME), false);
        create_alias_file(DEFAULT_ALIAS_NAME);
        assert!(alias_exists(DEFAULT_ALIAS_NAME));

        let mut args = args!["-cuh", "http://test.com"];
        let _: Box<dyn Command<TestValidOsDirs, &mut Vec<u8>, &mut Vec<u8>>> = args.command(&TestValidOsDirs::new()).expect("Cannot execute default_alias");
        assert_eq!(args, vec!["-v", "-c", "-cuh", "http://test.com"]);
    }

    #[test]
    fn empty_alias() {
        setup();
        assert_eq!(alias_exists(EMPTY_ALIAS_NAME), false);
        create_empty_alias_file(EMPTY_ALIAS_NAME);
        assert!(alias_exists(EMPTY_ALIAS_NAME));

        let mut args = args![arg_alias!(EMPTY_ALIAS_NAME), "-cuh", "http://test.com"];
        let _: Box<dyn Command<TestValidOsDirs, &mut Vec<u8>, &mut Vec<u8>>> = args.command(&TestValidOsDirs::new()).expect("Cannot execute empty_alias");
        assert_eq!(args, vec!["-cuh", "http://test.com"]);
    }

    #[test]
    fn custom_alias_with_one_arg() {
        setup();
        assert_eq!(alias_exists(CUSTOM_ALIAS_NAME_1), false);
        create_alias_file_with_args(CUSTOM_ALIAS_NAME_1, "-cUs");
        assert!(alias_exists(CUSTOM_ALIAS_NAME_1));

        let mut args = args![arg_alias!(CUSTOM_ALIAS_NAME_1), "-cUh", "http://test.com"];
        let _: Box<dyn Command<TestValidOsDirs, &mut Vec<u8>, &mut Vec<u8>>> = args.command(&TestValidOsDirs::new()).unwrap();
        assert_eq!(args, vec!["-cUs", "-cUh", "http://test.com"]);
    }

    #[test]
    fn custom_alias_with_multi_args() {
        setup();
        assert_eq!(alias_exists(CUSTOM_ALIAS_NAME_2), false);
        create_alias_file_with_args(CUSTOM_ALIAS_NAME_2, "-UhH\nX-Key:Val");
        assert!(alias_exists(CUSTOM_ALIAS_NAME_2));

        let mut args = args![arg_alias!(CUSTOM_ALIAS_NAME_2), "-cUh", "http://test.com"];
        let _: Box<dyn Command<TestValidOsDirs, &mut Vec<u8>, &mut Vec<u8>>> = args.command(&TestValidOsDirs::new()).unwrap();
        assert_eq!(args, vec!["-UhH", "X-Key:Val", "-cUh", "http://test.com"]);
    }

    #[test]
    fn error_config() {
        let mut args = args![arg_alias!("error"), "-cuh", "http://test.com"];
        let res: Result<Box<dyn Command<TestValidOsDirs, &mut Vec<u8>, &mut Vec<u8>>>> = args.command(&TestValidOsDirs::new());
        assert!(res.is_err());
        // FIXME Checks the error
    }
}
