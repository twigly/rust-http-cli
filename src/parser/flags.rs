use super::Error;
use crate::core::Flags;
use regex::Regex;

impl Default for Flags {
    fn default() -> Self {
        Self {
            show_version: false,
            show_help: false,

            https: false,
            http: false,
            use_color: true,
            show_direction: false,

            as_json: false,
            as_form: false,

            show_request_url: false,
            show_request_headers: false,
            show_request_compact: false,
            show_request_body: false,

            show_response_status: false,
            show_response_headers: false,
            show_response_compact: false,
            show_response_body: true,
        }
    }
}

impl Flags {
    pub fn new(output_redirected: bool) -> Flags {
        Flags {
            use_color: !output_redirected,
            ..Default::default()
        }
    }

    pub fn push(&mut self, flag: &str) -> Result<(), Error> {
        match flag {
            "--version" => self.show_version = true,
            "--help" => self.show_help = true,
            "-u" | "--url" => self.show_request_url = true,
            "-s" | "--status" => self.show_response_status = true,
            "-d" | "--direction" => self.show_direction = true,
            "--pretty=c" | "--pretty=color" => self.use_color = true,
            "--json" => self.as_json = true,
            "--form" => self.as_form = true,
            "--http" => {
                self.http = true;
                if self.is_contradictory_scheme() {
                    return Err(Error::ContradictoryScheme);
                }
            }
            "--https" | "--ssl" => {
                self.https = true;
                if self.is_contradictory_scheme() {
                    return Err(Error::ContradictoryScheme);
                }
            }
            "--headers" => {
                self.show_request_headers = true;
                self.show_response_headers = true;
            }
            "-H" | "--req-headers" => self.show_request_headers = true,
            "-h" | "--header" => self.show_response_headers = true,
            "-B" | "--req-body" => self.show_request_body = true,
            "-b" | "--body" => self.show_response_body = true,
            "-C" | "--req-compact" => self.show_request_compact = true,
            "-c" | "--compact" => self.show_response_compact = true,
            _ => {
                let has_valid_compact_flags = self.extract_compact_flags(flag);
                if !has_valid_compact_flags {
                    return Err(Error::InvalidFlag(flag.to_string()));
                }
            }
        };
        Ok(())
    }

    fn extract_compact_flags(&mut self, flag: &str) -> bool {
        // FIXME Need something like "-no-bBH..." to set the related flags to false
        let valid = Regex::new(r"^\-[cCdushHbB]*$").unwrap().is_match(flag);
        if valid {
            if flag.contains("c") {
                self.show_response_compact = true;
            }
            if flag.contains("C") {
                self.show_request_compact = true;
            }
            if flag.contains("d") {
                self.show_direction = true;
            }
            if flag.contains("u") {
                self.show_request_url = true;
            }
            if flag.contains("s") {
                self.show_response_status = true;
            }
            if flag.contains("H") {
                self.show_request_headers = true;
            }
            if flag.contains("h") {
                self.show_response_headers = true;
            }
            if flag.contains("b") {
                self.show_response_body = true;
            }
            if flag.contains("B") {
                self.show_request_body = true;
            }
        }
        valid
    }

    fn is_contradictory_scheme(&self) -> bool {
        self.http && self.https
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Flags};

    macro_rules! flag {
        () => {{
            Flags::new(false)
        }};
        ( $( $elem:expr ),* ) => {
            {
                let mut temp_flags = Flags::new(false);
                $(
                    let _ = temp_flags.push($elem);
                )*
                temp_flags
            }
        };
    }

    #[test]
    fn valid_scheme() {
        let flags = flag!["--http"];
        assert_eq!(flags.http, true);

        let flags = flag!["--https"];
        assert_eq!(flags.https, true);
    }

    #[test]
    fn contradictory_scheme() {
        let mut flags = flag![];
        let _ = flags.push("--http");
        let res = flags.push("--https");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::ContradictoryScheme);

        let mut flags = flag![];
        let _ = flags.push("--https");
        let res = flags.push("--http");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::ContradictoryScheme);

        let mut flags = flag!["-H", "-h"];
        let _ = flags.push("--https");
        let res = flags.push("--http");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::ContradictoryScheme);
    }

    #[test]
    fn compact_flags() {
        let flags = flag!["-hH"];
        assert_eq!(flags.show_request_headers, true);
        assert_eq!(flags.show_response_headers, true);

        let flags = flag!["-Hh"];
        assert_eq!(flags.show_request_headers, true);
        assert_eq!(flags.show_response_headers, true);

        let flag = "-hHa";
        let mut flags = flag![];
        let res = flags.push(flag);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::InvalidFlag(flag.into()));

        let flag = "-ahH";
        let mut flags = flag![];
        let res = flags.push(flag);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::InvalidFlag(flag.into()));
    }
}
