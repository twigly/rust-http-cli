#[cfg(not(feature = "screencast"))]
mod busy;
mod error;
mod exit;
mod output;
mod usage;
mod version;

use crate::core::{Error, Mode};
use crate::parser;
use crate::request;

#[cfg(not(feature = "screencast"))]
use busy::Spinner;

pub struct App {
    #[cfg(not(feature = "screencast"))]
    busy: Spinner,
}

impl App {
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "screencast"))]
            busy: busy::Spinner::new(),
        }
    }

    pub fn exit(self, err: Option<Error>) {
        #[cfg(not(feature = "screencast"))]
        self.busy.done();
        match err {
            Some(err) => {
                // usage::show(&err);
                error::show(&err);
                exit::error(err);
            }
            None => exit::success(),
        }
    }

    pub fn run(&self, args: &[String]) -> Result<(), Error> {
        let args = parser::execute(&args)?;
        match args.mode() {
            Mode::Help => {
                #[cfg(not(feature = "screencast"))]
                self.busy.clone().done();
                usage::help()
            }
            Mode::Version => {
                #[cfg(not(feature = "screencast"))]
                self.busy.clone().done();
                version::show()
            }
            Mode::Run => {
                {
                    let mut headers = args.headers.borrow_mut();
                    request::headers::upgrade(&args, &mut headers);
                }
                let headers = args.headers.borrow();
                let req_number = 0u8;
                let mut response = request::execute(&args, req_number, &headers)?;
                #[cfg(not(feature = "screencast"))]
                self.busy.clone().done();
                output::render(&args, req_number, &mut response);
            }
        }
        Ok(())
    }
}
