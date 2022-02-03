use super::number::Number;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    // Array(Vec<Value>),
}

impl<'a> From<&'a str> for Value {
    fn from(value: &str) -> Self {
        match value {
            "true" | "y" => Value::Bool(true),
            "false" | "n" => Value::Bool(false),
            "" => Value::Null,
            _ => match Number::from_str(value) {
                Some(num) => Value::Number(num),
                _ => Value::String(value.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    macro_rules! assert_value_eq {
        ($value:expr, $expected:expr) => {
            let value: Value = $value.into();
            assert_eq!(value, $expected)
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
    fn detect_type_from_str() {
        assert_value_eq!("true", Value::Bool(true));
        assert_value_eq!("y", Value::Bool(true));
        assert_value_eq!("false", Value::Bool(false));
        assert_value_eq!("n", Value::Bool(false));

        assert_value_eq!("$", value_string!("$"));
        assert_value_eq!("hello", value_string!("hello"));

        assert_value_eq!("1", value_number!(1));
        assert_value_eq!("123", value_number!(123));
        assert_value_eq!("123.456", value_number!((123.456)));
        assert_value_eq!("-123.456", value_number!((-123.456)));

        assert_value_eq!("", Value::Null);
    }
}
