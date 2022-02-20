use super::alias_filename;
use crate::commands::alias::error::ErrorKind;
use crate::commands::alias::Error;
use crate::commands::alias::Result;
use crate::core::Args;
use crate::shell::os::OsDirs;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub fn save<OD: OsDirs>(os_dirs: &OD, name: &str, args: &Args) -> Result<()> {
    match os_dirs.app_path(&alias_filename(name)) {
        Some(path) => match get_config_directory_error_if_any(os_dirs) {
            None => {
                let file = create_alias_file(name, &path)?;
                write(args, &file)
            }
            Some(err_kind) => Err(Error::CannotCreateAlias(name.to_string(), err_kind)),
        },
        None => Err(Error::CannotCreateAlias(name.to_string(), ErrorKind::ConfigDirectoryNotFound)),
    }
}

pub fn delete<OD: OsDirs>(os_dirs: &OD, name: &str) -> Result<()> {
    match os_dirs.app_path(&alias_filename(name)) {
        Some(path) => match get_config_directory_error_if_any(os_dirs) {
            None => match fs::remove_file(&path) {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::CannotDeleteAlias(name.to_string(), err.kind().into())),
            },
            Some(err_kind) => Err(Error::CannotDeleteAlias(name.to_string(), err_kind)),
        },
        None => Err(Error::CannotDeleteAlias(name.to_string(), ErrorKind::ConfigDirectoryNotFound)),
    }
}

fn write<R: io::Write>(args: &Args, writer: R) -> Result<()> {
    let mut buffer = BufWriter::new(writer);
    buffer.write_all(args.join("\n").as_bytes()).expect("Unable to write data");
    Ok(())
}

fn get_config_directory_error_if_any<OD: OsDirs>(os_dirs: &OD) -> Option<ErrorKind> {
    match os_dirs.config_directory() {
        Some(dir) => {
            if Path::exists(&dir) {
                None
            } else {
                Some(ErrorKind::InvalidConfigDirectory)
            }
        }
        None => Some(ErrorKind::ConfigDirectoryNotFound),
    }
}

fn create_alias_file(name: &str, path: &Path) -> Result<File> {
    match path.parent() {
        Some(dir) => {
            if !Path::exists(dir) && fs::create_dir(dir).is_err() {
                return Err(Error::CannotCreateAlias(name.to_string(), ErrorKind::CannotCreateAppConfigDirectory));
            }
            let file = File::create(&path)?;
            Ok(file)
        }
        None => Err(Error::CannotCreateAlias(name.to_string(), ErrorKind::CannotCreateAppConfigDirectory)),
    }
}
