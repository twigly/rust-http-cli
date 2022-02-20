mod app;
mod commands;
mod core;
mod items;
mod macros;
mod parser;
mod request;
mod shell;
#[cfg(test)]
mod test;
mod theme;

use crate::app::App;
use shell::os::DefaultOsDirs;
use shell::Shell;
use std::env;
use std::io;

fn main() {
    let mut os_args = env::args().skip(1).collect::<Vec<_>>();

    // let stdout = io::stdout();
    // let out = io::BufWriter::new(stdout.lock());
    // let stderr = io::stderr();
    // let err = io::BufWriter::new(stderr.lock());
    // let mut shell = Shell::new(out, err);

    let out = io::stdout();
    let err = io::stderr();
    let os_dirs = DefaultOsDirs::new();
    let mut shell = Shell::new(&os_dirs, out, err);

    let mut app = App::new(&mut shell);
    match app.run(&mut os_args) {
        Ok(_) => app.exit(None),
        Err(err) => app.exit(Some(err)),
    }
}
