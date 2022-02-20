use super::alias_filename;
use super::{iter::FilterOkTrait, DEFAULT_ALIAS_NAME};
use crate::commands::alias::{Error, Result};
use crate::core::Args;
use crate::shell::os::OsDirs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn from_default<OD: OsDirs>(os_dirs: &OD) -> Result<Args> {
    from_name(os_dirs, DEFAULT_ALIAS_NAME)
}

pub fn from_name<OD: OsDirs>(os_dirs: &OD, name: &str) -> Result<Args> {
    match os_dirs.app_path(&alias_filename(name)) {
        Some(path) => match from_path(&path) {
            Ok(args) => Ok(args),
            Err(err) => Err(Error::CannotLoadAlias(name.into(), err.kind().into())),
        },
        None => Ok(Args::new()),
    }
}

pub fn from_path(path: &Path) -> std::result::Result<Args, io::Error> {
    match File::open(&path) {
        Ok(file) => from_reader(&file),
        Err(err) => Err(err),
    }
}

fn from_reader<R: io::Read>(reader: R) -> std::result::Result<Args, io::Error> {
    let buffer = BufReader::new(reader);
    match buffer.lines().filter_ok(|arg| !arg.is_empty()).collect() {
        Ok(args) => Ok(args),
        Err(err) => Err(err),
    }
}

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands::alias::error::ErrorKind,
        test::{alias::*, os::TestValidOsDirs},
    };

    mod basic {
        use super::*;

        #[test]
        fn lines() {
            let args = from_reader(&b"-cushH\nX-KEY-1:val1\nX-KEY-2:val2"[..]).unwrap();
            assert_eq!(args, vec!["-cushH", "X-KEY-1:val1", "X-KEY-2:val2"]);
        }

        #[test]
        fn empty_lines() {
            let args = from_reader(&b"-cushH\n\n\nX-KEY-1:val1\nX-KEY-2:val2\n\n\n"[..]).unwrap();
            assert_eq!(args, vec!["-cushH", "X-KEY-1:val1", "X-KEY-2:val2"]);
        }

        #[test]
        fn error_if_invalid_characters() {
            let res = from_reader(&b"\xAA-cushH\nX-KEY-1:val1\nX-KEY-2:val2"[..]);
            assert!(res.is_err());
            // assert_eq!(res.unwrap_err(), Error::Config("testlines".into(), "testlines".into()));
        }
    }

    mod default_alias {
        use super::*;

        #[test]
        fn lines_from_default_alias() {
            setup();
            assert_eq!(alias_exists(DEFAULT_ALIAS_NAME), false);
            create_alias_file(DEFAULT_ALIAS_NAME);
            assert!(alias_exists(DEFAULT_ALIAS_NAME));

            let args = from_default(&TestValidOsDirs::new()).unwrap();
            assert_eq!(args, vec!["-v", "-c"]);
        }

        #[test]
        fn error_if_default_alias_missing() {
            setup();

            let res = from_default(&TestValidOsDirs::new());
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), Error::CannotLoadAlias(DEFAULT_ALIAS_NAME.into(), ErrorKind::AliasFileNotFound));
        }
    }

    mod custom_alias {
        use super::*;

        #[test]
        fn lines_from_empty_alias() {
            setup();
            assert_eq!(alias_exists(EMPTY_ALIAS_NAME), false);
            create_empty_alias_file(EMPTY_ALIAS_NAME);
            assert!(alias_exists(EMPTY_ALIAS_NAME));

            let args = from_name(&TestValidOsDirs::new(), EMPTY_ALIAS_NAME).unwrap();
            assert_eq!(args, Vec::<String>::new());
        }

        #[test]
        fn lines_from_1arg_alias() {
            setup();
            assert_eq!(alias_exists(CUSTOM_ALIAS_NAME_1), false);
            create_alias_file_with_args(CUSTOM_ALIAS_NAME_1, "-cUs");
            assert!(alias_exists(CUSTOM_ALIAS_NAME_1));

            let args = from_name(&TestValidOsDirs::new(), CUSTOM_ALIAS_NAME_1).unwrap();
            assert_eq!(args, vec!["-cUs"]);
        }

        #[test]
        fn lines_from_2args_alias() {
            setup();
            assert_eq!(alias_exists(CUSTOM_ALIAS_NAME_2), false);
            create_alias_file_with_args(CUSTOM_ALIAS_NAME_2, "-UhH\nX-Key:Val");
            assert!(alias_exists(CUSTOM_ALIAS_NAME_2));

            let args = from_name(&TestValidOsDirs::new(), CUSTOM_ALIAS_NAME_2).unwrap();
            assert_eq!(args, vec!["-UhH", "X-Key:Val"]);
        }

        #[test]
        fn error_if_no_alias_file() {
            setup();
            let res = from_name(&TestValidOsDirs::new(), "non-existing-alias");
            assert!(res.is_err());
            assert_eq!(res.unwrap_err(), Error::CannotLoadAlias("non-existing-alias".into(), ErrorKind::AliasFileNotFound));
        }
    }
}
