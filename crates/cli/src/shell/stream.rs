use atty::{is, Stream};

pub fn is_stdout() -> bool {
    is(Stream::Stdout)
}

pub fn is_stdin() -> bool {
    is(Stream::Stdin)
}
