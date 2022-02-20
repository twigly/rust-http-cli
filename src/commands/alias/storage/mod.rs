mod iter;
pub(crate) mod load;
pub(crate) mod show;
pub(crate) mod store;

use crate::core::Result;
pub use load::from_default;
pub use load::from_name;

pub const DEFAULT_ALIAS_NAME: &str = "default";

pub const ALIAS_FILENAME_PREFIX: &str = ".rh_";
pub const ALIAS_FILENAME_SUFFIX: &str = "_rc";

pub trait AliasArgItem {
    fn enrich_with_alias(&mut self) -> Result<()>;
}

fn alias_filename(name: &str) -> String {
    format!("{}{}{}", ALIAS_FILENAME_PREFIX, name, ALIAS_FILENAME_SUFFIX)
}
