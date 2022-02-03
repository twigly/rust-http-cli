mod app;
mod config;
mod core;
mod file;
mod items;
mod parser;
mod print;
mod request;
mod terminal;
#[cfg(test)]
mod test;
mod theme;
mod util;

use crate::app::App;
use std::env;

fn main() {
    let os_args = env::args().skip(1).collect::<Vec<_>>();
    let app = App::new();
    match app.run(&os_args) {
        Ok(_) => app.exit(None),
        Err(err) => app.exit(Some(err)),
    }
}
