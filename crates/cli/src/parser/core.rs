use regex::Regex;

pub const RAW_FLAG: &str = "--raw=";
pub const CAFILE_FLAG: &str = "--cafile=";

pub trait ArgDetection {
    fn is_raw_flag(&self) -> bool;
    fn is_cafile_flag(&self) -> bool;
    fn is_flag(&self) -> bool;
    fn is_header(&self) -> bool;
    fn is_item(&self) -> bool;

    fn is_likely_url(&self) -> bool;
    fn is_very_likely_url(&self) -> bool;
    fn is_url(&self) -> bool;
}

impl ArgDetection for String {
    fn is_raw_flag(&self) -> bool {
        self.starts_with(RAW_FLAG)
    }
    fn is_cafile_flag(&self) -> bool {
        self.starts_with(CAFILE_FLAG)
    }
    fn is_flag(&self) -> bool {
        self.starts_with('-')
    }
    fn is_header(&self) -> bool {
        match self.chars().next() {
            Some(first_char) => first_char.is_ascii_alphanumeric() && self.contains(':') && !self.contains('='),
            None => false,
        }
    }
    fn is_item(&self) -> bool {
        !self.starts_with('=') && !self.starts_with('/') && !self.starts_with(':') && self.contains('=')
    }

    fn is_likely_url(&self) -> bool {
        !self.is_flag()
    }
    fn is_very_likely_url(&self) -> bool {
        Regex::new(r"^\w+[-\.\w]*:+\d{1,5}$|^\w+[-\.\w]*$").unwrap().is_match(self)
    }
    fn is_url(&self) -> bool {
        // FIXME Add IPv6 and IPv4 detection
        self.starts_with("http://") || self.starts_with("https://") || self.starts_with(':') || self.starts_with('/')
    }
}

// UNIT TESTS /////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::ArgDetection;

    macro_rules! arg {
        ($val:expr) => {
            String::from($val)
        };
    }

    #[test]
    fn raw_flag() {
        assert!(arg!("--raw=").is_raw_flag());
        assert!(arg!("--raw=data").is_raw_flag());
    }
    #[test]
    fn not_raw_flag() {
        assert!(!arg!("--raw").is_raw_flag());
        assert!(!arg!("-raw").is_raw_flag());
        assert!(!arg!("-raw=").is_raw_flag());
        assert!(!arg!("-raw=data").is_raw_flag());
    }

    #[test]
    fn ca_flag() {
        assert!(arg!("--cafile=").is_cafile_flag());
        assert!(arg!("--cafile=path").is_cafile_flag());
    }
    #[test]
    fn not_ca_flag() {
        assert!(!arg!("--cafile").is_cafile_flag());
        assert!(!arg!("-cafile").is_cafile_flag());
        assert!(!arg!("-cafile=").is_cafile_flag());
        assert!(!arg!("-cafile=data").is_cafile_flag());
    }

    #[test]
    fn flag() {
        assert!(arg!("-").is_flag());
        assert!(arg!("-raw").is_flag());
        assert!(arg!("--raw=").is_flag());
        assert!(arg!("--raw=data").is_flag());
        assert!(arg!("-aBc").is_flag());
    }
    #[test]
    fn not_flag() {
        assert!(!arg!("not-a-flag-").is_flag());
    }

    #[test]
    fn header() {
        assert!(arg!("Key:Value").is_header());
        assert!(arg!("Key-1:Value/hello/bye").is_header());
    }
    #[test]
    fn not_header() {
        assert!(!arg!(".Key:Value").is_header());
        assert!(!arg!(":Key:Value").is_header());
        assert!(!arg!("/Key:Value").is_header());
        assert!(!arg!("Key:SubKey=Value").is_header());
    }

    #[test]
    fn item() {
        assert!(arg!("Key=Value").is_item());
        assert!(arg!(".Key=.Value").is_item());
        assert!(arg!("Key=Value:SubValue").is_item());
    }
    #[test]
    fn not_item() {
        assert!(!arg!(":Key=Value").is_item());
        assert!(!arg!("/Key=Value").is_item());
        assert!(!arg!("=Key=Value").is_item());
    }

    #[test]
    fn likely_url() {
        assert!(arg!("anything").is_likely_url());
    }
    #[test]
    fn not_likely_url() {
        assert_eq!(arg!("--a-flag-is-not-likely-an-url").is_likely_url(), false);
    }

    #[test]
    fn very_likely_url() {
        assert!(arg!("localhost").is_very_likely_url());
        assert!(arg!("localhost:1").is_very_likely_url());
        assert!(arg!("localhost:12").is_very_likely_url());
        assert!(arg!("localhost:123").is_very_likely_url());
        assert!(arg!("localhost:1234").is_very_likely_url());
        assert!(arg!("localhost:12345").is_very_likely_url());
        assert!(arg!("anything").is_very_likely_url());
        assert!(arg!("anything:8080").is_very_likely_url());
        assert!(arg!("my-hostname").is_very_likely_url());
        assert!(arg!("my-hostname:12345").is_very_likely_url());
        assert!(arg!("test.com").is_very_likely_url());
        assert!(arg!("test.com:80").is_very_likely_url());
        assert!(arg!("test.co.uk").is_very_likely_url());
        assert!(arg!("test.co.uk:443").is_very_likely_url());
        assert!(arg!("a.uk").is_very_likely_url());
        assert!(arg!("b.uk:1").is_very_likely_url());
    }

    #[test]
    fn not_very_likely_url() {
        assert_eq!(arg!("anything:hi").is_very_likely_url(), false);
        assert_eq!(arg!("anything:808055").is_very_likely_url(), false);
        assert_eq!(arg!("my-hostname:hello").is_very_likely_url(), false);
        assert_eq!(arg!("my-hostname:123456").is_very_likely_url(), false);
        assert_eq!(arg!(":test.com").is_very_likely_url(), false);
        assert_eq!(arg!("test.com:abcdef").is_very_likely_url(), false);
        assert_eq!(arg!("test.com:654321").is_very_likely_url(), false);
        assert_eq!(arg!("test.co.uk:qwerty").is_very_likely_url(), false);
        assert_eq!(arg!("-test.co.uk").is_very_likely_url(), false);
        assert_eq!(arg!(".test.co.uk").is_very_likely_url(), false);
        assert_eq!(arg!("@test.co.uk").is_very_likely_url(), false);
        assert_eq!(arg!("/test.co.uk").is_very_likely_url(), false);
        assert_eq!(arg!("*test.co.uk").is_very_likely_url(), false);
    }

    #[test]
    fn url() {
        assert!(arg!("http://test.com").is_url());
        assert!(arg!("https://test.com").is_url());
        assert!(arg!("/path/hello?r=y").is_url());
        assert!(arg!("/").is_url());
        assert!(arg!("/path").is_url());
        assert!(arg!(":").is_url());
        assert!(arg!(":9200").is_url());
    }
    #[test]
    fn not_url() {
        assert_eq!(arg!("not-anything").is_url(), false);
        assert_eq!(arg!("--a-flag-is-not-an-url").is_url(), false);
    }
}
