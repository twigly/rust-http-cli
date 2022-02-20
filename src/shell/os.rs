use std::path::PathBuf;

use crate::rh_name;

pub trait OsDirs {
    fn app_path(&self, filename: &str) -> Option<PathBuf>;
    fn app_config_directory(&self) -> Option<PathBuf>;
    fn config_directory(&self) -> Option<PathBuf>;
}

pub struct DefaultOsDirs;

impl DefaultOsDirs {
    pub fn new() -> Self {
        Self {}
    }
}

impl OsDirs for DefaultOsDirs {
    fn app_path(&self, filename: &str) -> Option<PathBuf> {
        self.app_config_directory().map(|path| path.join(filename))
    }

    fn app_config_directory(&self) -> Option<PathBuf> {
        self.config_directory().map(|path| path.join(rh_name!()))
    }

    fn config_directory(&self) -> Option<PathBuf> {
        dirs::config_dir()
    }
}
