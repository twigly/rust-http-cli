pub const RAW_FLAG: &str = "--raw=";

pub trait ArgDetection {
    fn is_raw_flag(&self) -> bool;
    fn is_flag(&self) -> bool;
    fn is_header(&self) -> bool;
    fn is_item(&self) -> bool;

    fn is_likely_url(&self) -> bool;
    fn is_url(&self) -> bool;
}

impl ArgDetection for String {
    fn is_raw_flag(&self) -> bool {
        self.starts_with(RAW_FLAG)
    }
    fn is_flag(&self) -> bool {
        self.starts_with("-")
    }
    fn is_header(&self) -> bool {
        match self.chars().nth(0) {
            Some(first_char) => {
                first_char.is_ascii_alphanumeric() && self.contains(":") && !self.contains("=")
            }
            None => false,
        }
    }
    fn is_item(&self) -> bool {
        !self.starts_with("=")
            && !self.starts_with("/")
            && !self.starts_with(":")
            && self.contains("=")
    }

    fn is_likely_url(&self) -> bool {
        !self.is_flag()
    }
    fn is_url(&self) -> bool {
        // FIXME Add IPv6 and IPv4 detection
        self.starts_with("http://")
            || self.starts_with("https://")
            || self.starts_with(":")
            || self.starts_with("/")
    }
}

#[cfg(test)]
mod tests {
    use super::ArgDetection;

    macro_rules! flag {
        ($flag:expr) => {
            $flag.to_string()
        };
    }

    #[test]
    fn raw_flag() {
        let flag = flag!("--raw");
        assert!(!flag.is_raw_flag());
        let flag = flag!("-raw");
        assert!(!flag.is_raw_flag());
        let flag = flag!("--raw=");
        assert!(flag.is_raw_flag());
        let flag = flag!("--raw=data");
        assert!(flag.is_raw_flag());
    }

    #[test]
    fn flag() {
        let flag = flag!("-");
        assert!(flag.is_flag());
        let flag = flag!("-raw");
        assert!(flag.is_flag());
        let flag = flag!("--raw=");
        assert!(flag.is_flag());
        let flag = flag!("--raw=data");
        assert!(flag.is_flag());
        let flag = flag!("-aBc");
        assert!(flag.is_flag());
        let flag = flag!("not-a-flag-");
        assert!(!flag.is_flag());
    }

    #[test]
    fn header() {
        let item = flag!("Key:Value");
        assert!(item.is_header());
        let item = flag!("Key-1:Value/hello/bye");
        assert!(item.is_header());
        let item = flag!(".Key:Value");
        assert!(!item.is_header());
        let item = flag!(":Key:Value");
        assert!(!item.is_header());
        let item = flag!("/Key:Value");
        assert!(!item.is_header());
        let item = flag!("Key:SubKey=Value");
        assert!(!item.is_header());
    }

    #[test]
    fn item() {
        let item = flag!("Key=Value");
        assert!(item.is_item());
        let item = flag!(".Key=.Value");
        assert!(item.is_item());
        let item = flag!("Key=Value:SubValue");
        assert!(item.is_item());
        let item = flag!(":Key=Value");
        assert!(!item.is_item());
        let item = flag!("/Key=Value");
        assert!(!item.is_item());
        let item = flag!("=Key=Value");
        assert!(!item.is_item());
    }

    #[test]
    fn likely_url() {
        let url = flag!("anything");
        assert!(url.is_likely_url());
        let url = flag!("--a-flag-is-not-likely-an-url");
        assert!(!url.is_likely_url());
    }

    #[test]
    fn url() {
        let url = flag!("http://test.com");
        assert!(url.is_url());
        let url = flag!("https://test.com");
        assert!(url.is_url());
        let url = flag!("/path/hello?r=y");
        assert!(url.is_url());
        let url = flag!("/");
        assert!(url.is_url());
        let url = flag!(":");
        assert!(url.is_url());
        let url = flag!(":9200");
        assert!(url.is_url());
        let url = flag!("not-anything");
        assert!(!url.is_url());
        let url = flag!("--a-flag-is-not-an-url");
        assert!(!url.is_url());
    }
}
