use crate::request::Method;

pub fn from_str(keyword: &str) -> Option<Method> {
    if !is_valid(keyword) {
        return None;
    }

    let method = reqwest::Method::from_bytes(keyword.as_bytes());
    match method {
        Ok(m) => Some(m),
        _ => None,
    }
}

fn is_valid(keyword: &str) -> bool {
    for c in keyword.chars() {
        if !c.is_uppercase() {
            return false;
        }
    }
    true
}

// fn is_standard(keyword: &str) -> bool {
//     let length = keyword.len();
//     if length != 3 && length != 4 && length != 5 && length != 6 && length != 7 {
//         return false;
//     }
//     [
//         "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "CONNECT", "PATCH", "TRACE",
//     ]
//     .contains(&keyword)
// }

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{from_str, Method};

    macro_rules! assert_standard_method_eq {
        ($method:expr, $expected:expr) => {
            assert_eq!(from_str($method).unwrap(), $expected)
        };
    }

    macro_rules! assert_standard_method_invalid {
        ($method:expr) => {
            assert!(from_str($method).is_none())
        };
    }

    #[test]
    fn standard() {
        assert_standard_method_eq!("GET", Method::GET);
        assert_standard_method_eq!("POST", Method::POST);
        assert_standard_method_eq!("PUT", Method::PUT);
        assert_standard_method_eq!("DELETE", Method::DELETE);
        assert_standard_method_eq!("HEAD", Method::HEAD);
        assert_standard_method_eq!("OPTIONS", Method::OPTIONS);
        assert_standard_method_eq!("CONNECT", Method::CONNECT);
        assert_standard_method_eq!("PATCH", Method::PATCH);
        assert_standard_method_eq!("TRACE", Method::TRACE);
    }

    #[test]
    fn custom() {
        assert_standard_method_eq!("HELLO", Method::from_bytes(b"HELLO").unwrap());
        assert_standard_method_eq!("WORLD", Method::from_bytes(b"WORLD").unwrap());
    }

    #[test]
    fn invalid() {
        assert_standard_method_invalid!("test");

        assert_standard_method_invalid!("get");
        assert_standard_method_invalid!("post");
        assert_standard_method_invalid!("put");
        assert_standard_method_invalid!("delete");
        assert_standard_method_invalid!("head");
        assert_standard_method_invalid!("options");
        assert_standard_method_invalid!("connect");
        assert_standard_method_invalid!("patch");
        assert_standard_method_invalid!("trace");
    }
}
