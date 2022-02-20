use super::os::app_config_directory_for_tests_only;
use crate::commands::alias::storage::{ALIAS_FILENAME_PREFIX, ALIAS_FILENAME_SUFFIX, DEFAULT_ALIAS_NAME};
use std::io::Write;
use std::{
    fs::{self, File},
    path::Path,
};

pub const CUSTOM_ALIAS_NAME_1: &str = "alias-one";
pub const CUSTOM_ALIAS_NAME_2: &str = "alias2";
pub const EMPTY_ALIAS_NAME: &str = "empty";

pub fn alias_filename(name: &str) -> String {
    format!(
        "{}/{}{}{}",
        app_config_directory_for_tests_only().display(),
        ALIAS_FILENAME_PREFIX,
        name,
        ALIAS_FILENAME_SUFFIX
    )
}

pub fn create_alias_file(name: &str) {
    create_alias_file_with_args(name, "-v\n-c");
}

pub fn create_alias_file_with_args(name: &str, args: &str) {
    let mut file = File::create(alias_filename(name)).expect("Cannot create the alias file");
    file.write_all(args.as_bytes()).expect("Cannot write content in the alias file");
}

pub fn create_empty_alias_file(name: &str) {
    let mut file = File::create(alias_filename(name)).expect("Cannot create the alias file");
    file.write_all("".as_bytes()).expect("Cannot write content in the alias file");
}

pub fn alias_exists(name: &str) -> bool {
    Path::new(&alias_filename(name)).exists()
}

pub fn delete_alias_file(name: &str) {
    let _ = fs::remove_file(alias_filename(name));
}

pub fn setup() {
    std::env::set_var("HOME", app_config_directory_for_tests_only());
    delete_alias_file(DEFAULT_ALIAS_NAME);
    delete_alias_file(CUSTOM_ALIAS_NAME_1);
    delete_alias_file(CUSTOM_ALIAS_NAME_2);
    delete_alias_file(EMPTY_ALIAS_NAME);
}
