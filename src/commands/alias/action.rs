use super::{Error, Result};
use crate::core::Args;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Action {
    Add,
    Delete,
    List,
    Help,
}

type IsValidAliasPtr = fn(arg: Option<&String>) -> bool;
type FixAliasPtr = fn(alias_name: &str) -> String;

pub fn get(
    args: &mut Args,
    is_valid_alias: IsValidAliasPtr,
    fix_alias: FixAliasPtr,
) -> Result<Action> {
    if let Some(potential_subcommand) = args.first() {
        if is_delete(potential_subcommand, args, is_valid_alias, fix_alias)? {
            args.remove(0);
            return Ok(Action::Delete);
        } else if is_list(potential_subcommand, args)? {
            args.remove(0);
            return Ok(Action::List);
        } else if is_add(potential_subcommand, args, is_valid_alias)? {
            args.remove(0);
            return Ok(Action::Add);
        } else if is_help(potential_subcommand)? {
            args.remove(0);
            return Ok(Action::Help);
        }
    }

    if args.is_empty() || (args.len() == 1 && is_valid_alias(args.get(0))) {
        Err(Error::NoArgs)
    } else {
        Ok(Action::Add)
    }
}

fn is_help(potential_subcommand: &str) -> Result<bool> {
    Ok(potential_subcommand == "--help" || potential_subcommand == "-h")
}
fn is_add(
    potential_subcommand: &str,
    args: &Args,
    is_valid_alias: IsValidAliasPtr,
) -> Result<bool> {
    if potential_subcommand == "--add" {
        if args.len() == 1 || (args.len() == 2 && is_valid_alias(args.get(1))) {
            return Err(Error::MissingArgsForAdd);
        } else {
            return Ok(true);
        }
    }
    Ok(false)
}
fn is_delete(
    potential_subcommand: &str,
    args: &Args,
    is_valid_alias: IsValidAliasPtr,
    fix_alias: FixAliasPtr,
) -> Result<bool> {
    if potential_subcommand == "--delete" || potential_subcommand == "--del" {
        if args.len() == 1 || (args.len() == 2 && is_valid_alias(args.get(1))) {
            Ok(true)
        } else {
            let first_arg = args.get(1).unwrap().to_lowercase();
            Err(Error::TooManyArgsForDelete(fix_alias(&first_arg)))
        }
    } else {
        Ok(false)
    }
}
fn is_list(potential_subcommand: &str, args: &Args) -> Result<bool> {
    if potential_subcommand == "--list" {
        if args.len() == 1 {
            Ok(true)
        } else {
            Err(Error::TooManyArgsForList)
        }
    } else {
        Ok(false)
    }
}

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{arg_alias, args, commands::ALIAS_NAME_PREFIX};

    fn is_valid(arg: Option<&String>) -> bool {
        match arg {
            Some(arg) => arg.starts_with(ALIAS_NAME_PREFIX),
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

    #[test]
    fn no_subcommand() {
        let mut args = args![];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::NoArgs);
    }

    #[test]
    fn invalid_add_subcommand_default_alias() {
        let mut args = args!["--add"];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingArgsForAdd);
    }

    #[test]
    fn invalid_add_subcommand_custom_alias() {
        let mut args = args!["--add", arg_alias!("an-alias")];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingArgsForAdd);
    }

    #[test]
    fn add_subcommand_default_alias() {
        let mut args = args!["--add", "-v"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Add);

        let mut args = args!["whatever"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Add);
    }

    #[test]
    fn add_subcommand_custom_alias() {
        let mut args = args!["--add", arg_alias!("an-alias"), "-v"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Add);

        let mut args = args![arg_alias!("an-alias"), "whatever"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Add);
    }

    #[test]
    fn invalid_delete_subcommand_uppercase() {
        let mut args = args!["--delete", "arg-ABC"];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            Error::TooManyArgsForDelete("arg-abc".into())
        );
    }
    #[test]
    fn invalid_delete_subcommand_lowercase() {
        let mut args = args!["--del", "arg-123"];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            Error::TooManyArgsForDelete("arg-123".into())
        );
    }

    #[test]
    fn invalid_delete_subcommand_error_without_prefix() {
        let mut args = args!["--del", arg_alias!("an-alias"), "arg-123"];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            Error::TooManyArgsForDelete("an-alias".into())
        );
    }

    #[test]
    fn delete_subcommand_default_alias() {
        let mut args = args!["--delete"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Delete);

        let mut args = args!["--del"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Delete);
    }

    #[test]
    fn delete_subcommand_custom_alias() {
        let mut args = args!["--delete", arg_alias!("an-alias")];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Delete);

        let mut args = args!["--del", arg_alias!("an-alias")];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Delete);
    }

    #[test]
    fn invalid_list_subcommand() {
        let mut args = args!["--list", "too-many-args"];
        let res = get(&mut args, is_valid, fix_alias_name);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::TooManyArgsForList);
    }

    #[test]
    fn list_subcommand() {
        let mut args = args!["--list"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::List);
    }

    #[test]
    fn help_subcommand_strict() {
        let mut args = args!["--help"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Help);

        let mut args = args!["-h"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Help);
    }

    #[test]
    fn help_subcommand_flexible() {
        let mut args = args!["--help", "blabla"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Help);

        let mut args = args!["-h", "blabla"];
        let subcommand = get(&mut args, is_valid, fix_alias_name).unwrap();
        assert_eq!(subcommand, Action::Help);
    }
}
