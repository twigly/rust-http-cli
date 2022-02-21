mod number;
mod ser;
mod value;

use crate::core::{Error, PushDataItem};
use std::collections::HashMap;
use value::Value;

pub type Items = HashMap<String, Value>;

const FORCE_STRING: &str = "/";

impl PushDataItem for Items {
    fn push(&mut self, item: &str) -> Result<(), Error> {
        match item.split_once("=") {
            Some(parts) => {
                let key = parts.0.to_string();
                if key.ends_with(FORCE_STRING) {
                    self.insert(key[..(key.len() - 1)].to_string(), Value::String(parts.1.to_string()))
                } else {
                    self.insert(key, parts.1.into())
                }
            }
            None => return Err(Error::InvalidItem(item.into())),
        };
        Ok(())
    }
}

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{Items, PushDataItem, Value, FORCE_STRING};

    macro_rules! assert_item_eq {
        ($item:expr, $key:expr, $value:expr) => {
            let mut items = Items::new();
            let _ = items.push($item.into());

            assert_eq!(items.len(), 1);
            assert_eq!(items.get(&$key.to_string()), Some(&$value))
        };
    }

    macro_rules! key_value_force_string {
        ($key:expr, $value:expr) => {
            format!("{}{}={}", $key, FORCE_STRING, $value).as_str()
        };
    }

    macro_rules! value_bool {
        ($value:expr) => {
            Value::Bool($value)
        };
    }
    macro_rules! value_string {
        ($value:expr) => {
            Value::String($value.to_string())
        };
    }
    macro_rules! value_number {
        ($value:expr) => {
            Value::Number($value.into())
        };
    }

    #[test]
    fn types() {
        assert_item_eq!("key=value", "key", value_string!("value"));
        assert_item_eq!("key=true", "key", value_bool!(true));
        assert_item_eq!("key=y", "key", value_bool!(true));
        assert_item_eq!("key=false", "key", value_bool!(false));
        assert_item_eq!("key=n", "key", value_bool!(false));

        assert_item_eq!("k|e|y=$true", "k|e|y", value_string!("$true"));
        assert_item_eq!("k.e.y=$false", "k.e.y", value_string!("$false"));
        assert_item_eq!(key_value_force_string!("k|e|y", "true"), "k|e|y", value_string!("true"));
        assert_item_eq!(key_value_force_string!("k|e|y", "y"), "k|e|y", value_string!("y"));
        assert_item_eq!(key_value_force_string!("k.e.y", "false"), "k.e.y", value_string!("false"));
        assert_item_eq!(key_value_force_string!("k|e|y", "n"), "k|e|y", value_string!("n"));
        assert_item_eq!(key_value_force_string!("@key", "hello"), "@key", value_string!("hello"));
        assert_item_eq!(key_value_force_string!("@key$", "hello"), "@key$", value_string!("hello"));

        assert_item_eq!("a=1", "a", value_number!(1));
        assert_item_eq!("bc=123", "bc", value_number!(123));
        assert_item_eq!("d-e=123.456", "d-e", value_number!((123.456)));
        assert_item_eq!("f_g=-123.456", "f_g", value_number!((-123.456)));
    }
}
