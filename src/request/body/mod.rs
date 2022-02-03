mod form;
mod json;

use crate::core::{ArgItems, Args};
use reqwest::blocking::RequestBuilder;

pub trait Body {
    fn body_if_items(self, args: &Args) -> RequestBuilder;
}

impl Body for RequestBuilder {
    fn body_if_items(self, args: &Args) -> RequestBuilder {
        match build_body(&args) {
            Some(body) => self.body(body),
            None => self,
        }
    }
}

fn build_body(args: &Args) -> Option<String> {
    if args.has_items() {
        if args.is_json() {
            Some(json::serialize(&args.items.borrow()).unwrap())
        } else {
            Some(form::serialize(&args.items.borrow()).unwrap())
        }
    } else if let Some(ref raw) = args.raw {
        Some(raw.clone())
    } else {
        None
    }
}
