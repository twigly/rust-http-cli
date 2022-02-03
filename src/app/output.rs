use std::borrow::Borrow;

use crate::core::Args;
use crate::file;
use crate::print;
use crate::request::Response;

pub fn render(args: &Args, req_number: u8, response: &mut Response) {
    if args.output_redirected && !args.flags.borrow().use_color {
        file::render(&args, response);
    } else {
        print::render(&args, req_number, response);
    }
}
