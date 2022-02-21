use rh::shell::os::DefaultOsDirs;
use rh::shell::Shell;
use std::env;
use std::io;
use std::process::exit;

fn main() {
    let mut os_args = env::args().skip(1).collect::<Vec<_>>();

    // let stdout = io::stdout();
    // let out = io::BufWriter::new(stdout.lock());
    // let stderr = io::stderr();
    // let err = io::BufWriter::new(stderr.lock());
    // let mut shell = Shell::new(out, err);

    let out = io::stdout();
    let err = io::stderr();
    let os_dirs = DefaultOsDirs::default();
    let mut shell = Shell::new(&os_dirs, out, err);

    let exit_code = rh::run(&mut os_args, &mut shell);
    exit(exit_code);
}
