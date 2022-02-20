use super::load::from_path;
use super::{ALIAS_FILENAME_PREFIX, ALIAS_FILENAME_SUFFIX};
use crate::commands::alias::{Error, Result};
use crate::core::Args;
use crate::shell::os::OsDirs;
use std::fs::{self, ReadDir};
use std::path::Path;

pub fn list<OD: OsDirs>(os_dirs: &OD) -> Result<Vec<Alias>> {
    match os_dirs.app_config_directory() {
        Some(path) => Ok(read_app_config(&path)?
            .filter(|res| res.is_ok())
            .map(|res| res.unwrap().path())
            .filter(|path| path.is_file() && is_alias_file(path))
            .map(|alias_path| alias(&alias_path))
            .collect()),
        None => Err(Error::CannotListAlias),
    }
}

fn read_app_config(path: &Path) -> Result<ReadDir> {
    match fs::read_dir(path) {
        Ok(res) => Ok(res),
        Err(_) => Err(Error::CannotListAlias),
    }
}

fn is_alias_file(path: &Path) -> bool {
    match path.file_name() {
        Some(filename) => {
            let filename = filename.to_str().unwrap();
            filename.starts_with(ALIAS_FILENAME_PREFIX) && filename.ends_with(ALIAS_FILENAME_SUFFIX)
        }
        None => false,
    }
}

fn alias(path: &Path) -> Alias {
    let filename = path.file_name().unwrap().to_string_lossy();
    let alias_start_pos_in_filename = ALIAS_FILENAME_PREFIX.len();
    let alias_end_pos_in_filename = filename.len() - ALIAS_FILENAME_SUFFIX.len();

    let args = match from_path(path) {
        Ok(args) => args,
        Err(_) => vec!["Can't load arguments".to_string()],
    };

    Alias {
        name: filename[alias_start_pos_in_filename..alias_end_pos_in_filename].to_string(),
        args,
    }
}

pub struct Alias {
    pub name: String,
    pub args: Args,
}
