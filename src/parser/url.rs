use regex::Regex;
use url::Url;

pub fn normalize(url: &str, default_scheme: &str, default_host: &str) -> String {
    let res = Url::parse(url);
    if res.is_ok() {
        if url.starts_with("http") {
            return url.into();
        } else {
            return format!("{}://{}", default_scheme, url);
        }
    }
    match url {
        ":" => format!("{}://{}", default_scheme, default_host),
        part if Regex::new(r"^://").unwrap().is_match(part) => {
            format!("{}://{}{}", default_scheme, default_host, &part[2..])
        }
        part if Regex::new(r"^:/").unwrap().is_match(part) => {
            format!("{}://{}{}", default_scheme, default_host, &part[1..])
        }
        part if Regex::new(r"^:\d").unwrap().is_match(part) => {
            format!("{}://{}{}", default_scheme, default_host, part)
        }
        part if Regex::new(r"^/").unwrap().is_match(part) => {
            format!("{}://{}{}", default_scheme, default_host, part)
        }
        _ => {
            if url.starts_with('/') {
                format!("{}://{}/{}", default_scheme, default_host, url)
            } else {
                format!("{}://{}", default_scheme, url)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::normalize;
    const DEFAULT_SCHEME: &str = "https";
    const DEFAULT_HOST: &str = "l-o-c-a-l-h-o-s-t";

    macro_rules! assert_normalize {
        ($url:expr, $expected:expr) => {
            assert_eq!(normalize($url, DEFAULT_SCHEME, DEFAULT_HOST), $expected)
        };
    }

    macro_rules! assert_normalize_with_defaults {
        ($url:expr, $expected:expr) => {
            assert_eq!(
                normalize($url, DEFAULT_SCHEME, DEFAULT_HOST),
                format!($expected, DEFAULT_SCHEME, DEFAULT_HOST)
            )
        };
    }

    macro_rules! assert_normalize_with_default_scheme {
        ($url:expr, $expected:expr) => {
            assert_eq!(
                normalize($url, DEFAULT_SCHEME, DEFAULT_HOST),
                format!($expected, DEFAULT_SCHEME)
            )
        };
    }

    macro_rules! assert_valid {
        ($url:expr) => {
            assert_eq!(normalize($url, DEFAULT_SCHEME, DEFAULT_HOST), $url)
        };
    }

    #[test]
    fn macro_assert_normalise() {
        assert_valid!("http://test.com");
        assert_normalize!("http://test.com", "http://test.com");
        assert_normalize_with_defaults!(
            format!("{}://{}", DEFAULT_SCHEME, DEFAULT_HOST).as_str(),
            "{}://{}"
        );
        assert_normalize_with_default_scheme!(
            format!("{}://{}", DEFAULT_SCHEME, "host-example.com").as_str(),
            "{}://host-example.com"
        );
    }

    #[test]
    fn host() {
        assert_normalize_with_default_scheme!("localhost", "{}://localhost");
        assert_normalize_with_default_scheme!("localhost:9200", "{}://localhost:9200");
        assert_normalize_with_default_scheme!("test.com", "{}://test.com");
        assert_normalize_with_default_scheme!("test.com:9200", "{}://test.com:9200");
        assert_normalize_with_default_scheme!("test.com/a?b=c", "{}://test.com/a?b=c");
        assert_normalize_with_default_scheme!("test.com:1024/a?b=c", "{}://test.com:1024/a?b=c");
        assert_normalize_with_default_scheme!("test.com/a/b/c", "{}://test.com/a/b/c");
        assert_normalize_with_default_scheme!("test.com:1024/a/b/c", "{}://test.com:1024/a/b/c");
    }

    #[test]
    fn default_host() {
        assert_normalize_with_defaults!(":", "{}://{}");
        assert_normalize_with_defaults!(":/", "{}://{}/");
        assert_normalize_with_defaults!(":/uri", "{}://{}/uri");
        assert_normalize_with_defaults!("://uri", "{}://{}/uri");
        assert_normalize_with_defaults!(":/uri/a/b/c", "{}://{}/uri/a/b/c");
        assert_normalize_with_defaults!(":/uri/a/b/c/d.html", "{}://{}/uri/a/b/c/d.html");
        assert_normalize_with_defaults!(":9000", "{}://{}:9000");
        assert_normalize_with_defaults!(":5000/", "{}://{}:5000/");
        assert_normalize_with_defaults!(":2000/uri", "{}://{}:2000/uri");
        assert_normalize_with_defaults!("/uri", "{}://{}/uri");
        assert_normalize_with_defaults!("/uri/a.jpeg", "{}://{}/uri/a.jpeg");
        assert_normalize_with_defaults!(DEFAULT_HOST, "{}://{}");
    }

    #[test]
    fn proper_urls() {
        assert_valid!("http://test.com");
        assert_valid!("https://test.com");
        assert_valid!("http://test.com:9000");
        assert_valid!("https://test.com:9000");
        assert_valid!("https://test.com:9000/a/b.html");
        assert_valid!("https://test.com:9000/a/b/");
        assert_valid!("https://test.com:9000/a/b.html?c=d");
    }
}
