use crate::core::{Workspace, WorkspaceData};
use crate::{rh_homepage, rh_name, rh_version};
use reqwest::header::{HeaderMap, HeaderValue};

use super::header;

pub fn upgrade(args: &Workspace, headers: &mut HeaderMap) {
    if args.is_json() {
        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.append(header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        }
        if !headers.contains_key(header::ACCEPT) {
            headers.append(header::ACCEPT, HeaderValue::from_str("application/json").unwrap());
        }
    }
    if args.is_form() && !headers.contains_key(header::CONTENT_TYPE) {
        headers.append(header::CONTENT_TYPE, HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
    }

    if !headers.contains_key(header::USER_AGENT) {
        headers.append(
            header::USER_AGENT,
            HeaderValue::from_str(&format!("{}/{} {}", rh_name!(), rh_version!(), rh_homepage!(),)).unwrap(),
        );
    }
}
