use serde_urlencoded;
use serde_urlencoded::ser::Error;

use crate::items::Items;

pub fn serialize(items: &Items) -> Result<String, Error> {
    serde_urlencoded::to_string(&items)
}
