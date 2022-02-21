use super::Error;

pub const SUCCESS: i32 = 0;

pub fn code_on_success() -> i32 {
    SUCCESS
}

pub fn code_on_error(err: Error) -> i32 {
    // FIXME All errors must have an exit code
    match err {
        Error::NoArgs => 100,
        Error::MissingUrl => 101,
        Error::ItemsAndRawMix => 200,
        Error::TooManyRaw => 201,
        Error::ContradictoryScheme => 301,
        #[cfg(feature = "alias")]
        Error::AliasCommand(_) => 950,
        #[cfg(feature = "alias")]
        Error::Alias(_) => 900,
        _ => 999,
    }
}
