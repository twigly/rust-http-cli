use std::{env, path::PathBuf};

use crate::shell::os::OsDirs;

const APP_NAME_FOR_TESTS_ONLY: &str = "rh-test";

pub fn app_config_directory_for_tests_only() -> PathBuf {
    config_directory_for_tests_only().join(APP_NAME_FOR_TESTS_ONLY)
}
pub fn config_directory_for_tests_only() -> PathBuf {
    env::temp_dir()
}

pub struct TestValidOsDirs;

impl TestValidOsDirs {
    pub fn new() -> Self {
        Self {}
    }
}

impl OsDirs for TestValidOsDirs {
    fn app_path(&self, filename: &str) -> Option<PathBuf> {
        self.app_config_directory().map(|path| path.join(filename))
    }

    fn app_config_directory(&self) -> Option<PathBuf> {
        self.config_directory().map(|path| path.join(APP_NAME_FOR_TESTS_ONLY))
    }

    fn config_directory(&self) -> Option<PathBuf> {
        Some(config_directory_for_tests_only())
    }
}

pub struct TestNoOsDirs;

impl TestNoOsDirs {
    pub fn new() -> Self {
        Self {}
    }
}

impl OsDirs for TestNoOsDirs {
    fn app_path(&self, filename: &str) -> Option<PathBuf> {
        self.app_config_directory().map(|path| path.join(filename))
    }

    fn app_config_directory(&self) -> Option<PathBuf> {
        self.config_directory()
    }

    fn config_directory(&self) -> Option<PathBuf> {
        None
    }
}

pub struct TestInvalidOsDirs;

impl TestInvalidOsDirs {
    pub fn new() -> Self {
        Self {}
    }
}

impl OsDirs for TestInvalidOsDirs {
    fn app_path(&self, filename: &str) -> Option<PathBuf> {
        self.app_config_directory().map(|path| path.join(filename))
    }

    fn app_config_directory(&self) -> Option<PathBuf> {
        self.config_directory()
    }

    fn config_directory(&self) -> Option<PathBuf> {
        Some(PathBuf::from("a-dir-that_does-not-exist"))
    }
}
