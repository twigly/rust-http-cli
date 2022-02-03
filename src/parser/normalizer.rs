use super::core::{ArgDetection, RAW_FLAG};
use super::headers::HeaderMap;
use super::method;
use super::url;
use crate::core::Flags;
use crate::core::{Error, PushItem};
use crate::items::Items;
use crate::request::Method;

#[cfg_attr(test, derive(Debug))]
pub struct Normalizer {
    pub urls: Vec<String>,
    method: Option<Method>,
    pub flags: Flags,
    pub headers: HeaderMap,
    pub items: Items,
    pub raw: Option<String>,
}

impl Normalizer {
    pub fn parse(
        args: &[String],
        output_redirected: bool,
        default_scheme: &str,
        default_host: &str,
    ) -> Result<Normalizer, Error> {
        let mut method: Option<Method> = None;
        let mut urls: Vec<String> = Vec::new();
        let mut flags = Flags::new(output_redirected);
        let mut headers = HeaderMap::new();
        let mut items = Items::new();
        let mut raw: Option<String> = None;
        let args_length = args.len();

        for arg_index in 0..args_length {
            let arg = args[arg_index].clone();

            if arg_index == 0 {
                method = method::from_str(&arg);
                if method.is_some() {
                    continue;
                }
            }

            if (method.is_some() && arg_index == 1) || (method.is_none() && arg_index == 0) {
                if arg.is_likely_url() {
                    urls.push(arg);
                    continue;
                }
            } else if arg.is_url() {
                urls.push(arg);
                continue;
            }

            if arg.is_raw_flag() {
                let raw_data = arg[RAW_FLAG.len()..].to_string();
                if raw.is_some() {
                    return Err(Error::TooManyRaw);
                }
                if raw_data.len() > 0 {
                    raw = Some(raw_data);
                }
            } else if arg.is_flag() {
                flags.push(&arg)?;
            } else {
                if arg.is_header() {
                    headers.push(&arg)?;
                } else if arg.is_item() {
                    items.push(&arg)?;
                } else {
                    if method.is_none() {
                        return Err(Error::Unexpected(arg));
                    }
                }
            }

            if flags.show_version || flags.show_help {
                break;
            }
        }

        if !flags.http && !flags.https {
            flags.http = true;
        }

        if urls.len() > 0 {
            let scheme = if flags.https {
                "https"
            } else if flags.http {
                "http"
            } else {
                default_scheme
            };
            for url in urls.iter_mut() {
                *url = url::normalize(&url, scheme, default_host);
            }
        }

        Ok(Normalizer {
            urls,
            method,
            flags,
            headers,
            items,
            raw,
        })
    }

    pub fn method(&mut self) -> Method {
        let method = self.method.take();
        match method {
            Some(method) => method,
            _ => {
                if self.has_input_data() {
                    Method::POST
                } else {
                    Method::GET
                }
            }
        }
    }

    pub fn has_input_data(&self) -> bool {
        self.items.len() > 0 || self.raw.is_some()
    }
}

// FIXME More tests (in particular if output_redirected=true)
#[cfg(test)]
mod tests {
    use super::{Error, Normalizer};
    const DEFAULT_SCHEME: &str = "http";
    const DEFAULT_HOST: &str = "l-o-c-a-l-h-o-s-t";

    macro_rules! assert_one_arg_url_eq {
        ($url:expr, $expected:expr) => {
            let args: Vec<String> = crate::args![$url];
            let mut normalizer =
                Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST).unwrap();
            assert!(normalizer.method() == Method::GET);
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(normalizer.urls[0], $expected);
        };
    }

    mod method {
        use super::{Error, Normalizer};
        use super::{DEFAULT_HOST, DEFAULT_SCHEME};
        use crate::request::Method;

        #[test]
        fn standard_method() {
            let args = crate::args!["HEAD", "localhost"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST).unwrap();
            assert_eq!(normalizer.method, Some(Method::HEAD));
            assert_eq!(normalizer.urls.len(), 1);
        }

        #[test]
        fn custom_method() {
            let args = crate::args!["HELLO", "localhost"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST).unwrap();
            assert_eq!(
                normalizer.method,
                Some(Method::from_bytes(b"HELLO").unwrap())
            );
            assert_eq!(normalizer.urls.len(), 1);
        }

        #[test]
        fn no_methods_because_lowercase() {
            let args = crate::args!["get", "localhost"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST);
            assert!(normalizer.is_err());
            assert_eq!(
                normalizer.unwrap_err(),
                Error::Unexpected("localhost".into())
            );
        }
    }

    mod urls {
        use super::{Error, Normalizer};
        use super::{DEFAULT_HOST, DEFAULT_SCHEME};
        use crate::request::Method;

        #[test]
        fn no_args() {
            let args = crate::args![];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST).unwrap();
            assert_eq!(normalizer.method, None);
            assert_eq!(normalizer.urls.len(), 0);
        }

        #[test]
        fn only_one_url_arg() {
            assert_one_arg_url_eq!("http://test.com", "http://test.com");
            assert_one_arg_url_eq!("test.com", &format!("{}://test.com", DEFAULT_SCHEME));
            assert_one_arg_url_eq!("test", &format!("{}://test", DEFAULT_SCHEME));
        }

        #[test]
        fn method_and_url() -> Result<(), Error> {
            let args = crate::args!["GET", "localhost"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(
                normalizer.urls[0],
                format!("{}://localhost", DEFAULT_SCHEME)
            );
            Ok(())
        }

        #[test]
        fn method_and_url_and_flag() -> Result<(), Error> {
            let args = crate::args!["GET", "localhost", "--headers"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(
                normalizer.urls[0],
                format!("{}://localhost", DEFAULT_SCHEME)
            );
            Ok(())
        }

        #[test]
        fn url_and_flag() -> Result<(), Error> {
            let args = crate::args!["localhost", "--headers"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(
                normalizer.urls[0],
                format!("{}://localhost", DEFAULT_SCHEME)
            );
            Ok(())
        }
    }

    mod flags {
        use super::{Error, Normalizer};
        use super::{DEFAULT_HOST, DEFAULT_SCHEME};
        use crate::request::Method;

        #[test]
        fn force_http() -> Result<(), Error> {
            let args: Vec<String> = crate::args!["GET", "test.com", "--http"];
            let mut normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert!(normalizer.method() == Method::GET);
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(normalizer.urls[0], "http://test.com");
            assert_eq!(normalizer.flags.http, true);
            assert_eq!(normalizer.flags.https, false);
            Ok(())
        }

        #[test]
        fn force_https() -> Result<(), Error> {
            let args: Vec<String> = crate::args!["GET", "test.com", "--https"];
            let mut normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert!(normalizer.method() == Method::GET);
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(normalizer.urls[0], "https://test.com");
            assert_eq!(normalizer.flags.http, false);
            assert_eq!(normalizer.flags.https, true);
            Ok(())
        }

        #[test]
        fn version() -> Result<(), Error> {
            let args: Vec<String> = crate::args!["--version"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert_eq!(normalizer.urls.len(), 0);
            assert_eq!(normalizer.method, None);
            assert_eq!(normalizer.flags.show_version, true);
            Ok(())
        }

        #[test]
        fn help() -> Result<(), Error> {
            let args: Vec<String> = crate::args!["--help"];
            let normalizer = Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST)?;
            assert_eq!(normalizer.urls.len(), 0);
            assert_eq!(normalizer.method, None);
            assert_eq!(normalizer.flags.show_help, true);
            Ok(())
        }
    }

    mod raw {
        use super::Normalizer;
        use super::{DEFAULT_HOST, DEFAULT_SCHEME};
        use crate::request::Method;

        #[test]
        fn raw_data() {
            let args: Vec<String> = crate::args!["test.com", "--raw=~data~"];
            let mut normalizer =
                Normalizer::parse(&args, false, DEFAULT_SCHEME, DEFAULT_HOST).unwrap();
            assert_eq!(normalizer.method(), Method::POST);
            assert_eq!(normalizer.urls.len(), 1);
            crate::assert_str_eq!(normalizer.urls[0], format!("{}://test.com", DEFAULT_SCHEME));
            assert_eq!(normalizer.raw, Some("~data~".to_string()));
            assert_eq!(normalizer.flags.as_json, false);
            assert_eq!(normalizer.flags.as_form, false);
        }
    }
}
