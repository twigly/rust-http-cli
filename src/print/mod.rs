mod core;
mod direction;
mod headers;
mod request;
mod response;

use self::core::HasRendered;
use crate::core::{Args, Error, Result};
use crate::request::Response;
use crate::terminal::TerminalError;

pub fn render(args: &Args, req_number: u8, response: &mut Response) -> Result<()> {
    let mut has_rendered = HasRendered::Nothing;

    has_rendered += request::render_method_and_url(args, req_number)?;
    has_rendered += headers::render_request(args, &args.headers.borrow(), true)?;
    core::render_newline_if(args,has_rendered)?;
    has_rendered = request::render_body(args)?;
    core::render_newline_if(args,has_rendered)?;

    let mut has_rendered = HasRendered::Nothing;
    has_rendered += response::render_status(args, response)?;
    has_rendered += headers::render_response(args, response.headers(), false)?;
    core::render_newline_if(args,has_rendered)?;

    response::render_body(args, response)?;

    Ok(())
}

impl From<TerminalError> for Error {
    fn from(_: TerminalError) -> Error {
        Error::Terminal
    }
}

// #[cfg(test)]
// mod tests {
//     use super::HasRendered;

//     #[test]
//     fn all_nothing() {
//         let mut has_rendered = HasRendered::Nothing;
//         has_rendered += HasRendered::Nothing;
//         has_rendered += HasRendered::Nothing;
//         assert_eq!(has_rendered + false, HasRendered::Nothing);
//         has_rendered += HasRendered::Nothing;
//         has_rendered += HasRendered::Nothing;
//         assert_eq!(has_rendered + false, HasRendered::Nothing);
//     }

//     #[test]
//     fn all_something_and_true() {
//         let mut has_rendered = HasRendered::Nothing;
//         has_rendered += HasRendered::Something;
//         has_rendered += HasRendered::Something;
//         assert_eq!(has_rendered + true, HasRendered::Something);
//         has_rendered += HasRendered::Something;
//         has_rendered += HasRendered::Something;
//         assert_eq!(has_rendered + true, HasRendered::Something);
//     }

//     #[test]
//     fn all_something_and_false() {
//         let mut has_rendered = HasRendered::Nothing;
//         has_rendered += HasRendered::Something;
//         has_rendered += HasRendered::Something;
//         assert_eq!(has_rendered + false, HasRendered::Nothing);
//         has_rendered += HasRendered::Something;
//         has_rendered += HasRendered::Something;
//         assert_eq!(has_rendered + false, HasRendered::Nothing);
//     }

//     #[test]
//     fn something_and_true() {
//         let mut has_rendered = HasRendered::Nothing;
//         has_rendered += HasRendered::Something;
//         has_rendered += HasRendered::Nothing;
//         assert_eq!(has_rendered + true, HasRendered::Something);
//         has_rendered += HasRendered::Nothing;
//         has_rendered += HasRendered::Nothing;
//         assert_eq!(has_rendered + true, HasRendered::Nothing);
//     }
// }
