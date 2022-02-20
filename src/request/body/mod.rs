mod form;
mod json;

use crate::core::{WorkspaceData, Workspace};
use reqwest::blocking::RequestBuilder;

pub trait Body {
    fn body_if_items(self, args: &Workspace) -> RequestBuilder;
}

impl Body for RequestBuilder {
    fn body_if_items(self, args: &Workspace) -> RequestBuilder {
        match build_body(args) {
            Some(body) => self.body(body),
            None => self,
        }
    }
}

fn build_body(args: &Workspace) -> Option<String> {
    if args.has_items() {
        if args.is_json() {
            Some(json::serialize(&args.items.borrow()).unwrap())
        } else {
            Some(form::serialize(&args.items.borrow()).unwrap())
        }
    } else {
        args.raw.as_ref().cloned()
    }
}
