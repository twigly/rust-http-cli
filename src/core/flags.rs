#[cfg_attr(test, derive(Debug))]
#[derive(Clone, Copy)]
pub struct Flags {
    pub show_version: bool,
    pub show_help: bool,
    pub show_short_help: bool,
    pub debug: bool,

    pub https: bool,
    pub http: bool,
    pub use_color: bool,
    pub show_direction: bool,

    pub as_json: bool,
    pub as_form: bool,

    pub show_request_url: bool,
    pub show_request_headers: bool,
    pub show_request_compact: bool,
    pub show_request_body: bool,

    pub show_response_status: bool,
    pub show_response_headers: bool,
    pub show_response_compact: bool,
    pub show_response_body: bool,
}

