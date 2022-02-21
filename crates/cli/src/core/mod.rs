mod error;
mod flags;
mod types;
mod workspace;

pub use error::Error;
pub use flags::Flags;
pub use types::{Args, HeaderMap, Mode, Result};
pub use workspace::Workspace;

pub trait PushDataItem {
    fn push(&mut self, item: &str) -> Result<()>;
}

pub trait WorkspaceData {
    fn is_json(&self) -> bool;
    fn is_form(&self) -> bool;
    fn has_items(&self) -> bool;
}
