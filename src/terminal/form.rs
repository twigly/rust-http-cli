use crate::items::Items;

pub fn println(items: &Items, _compact: bool) {
    match serde_urlencoded::to_string(&items) {
        Ok(buffer) => println!("{}", buffer),
        Err(_) => println!("Can't render the items as URL encoded"),
    };
}
