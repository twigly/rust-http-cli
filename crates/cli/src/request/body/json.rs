use serde_json::error::Error;

use crate::items::Items;

pub fn serialize(items: &Items) -> Result<String, Error> {
    serde_json::to_string(&items)
}
