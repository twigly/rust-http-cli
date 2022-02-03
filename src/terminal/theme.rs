use termcolor::Color as TermColor;

pub trait DefaultTheme {
    fn default() -> Self;
}

impl DefaultTheme for termcolor_json::Theme {
    fn default() -> Self {
        let mut theme = termcolor_json::Theme::none();

        theme.null_mut().set_fg(Some(TermColor::Cyan));
        theme.bool_mut().set_fg(Some(TermColor::Yellow));
        theme.number_mut().set_fg(Some(TermColor::Magenta));
        theme.string_mut().set_fg(Some(TermColor::Green));
        theme
            .object_key_mut()
            .set_fg(Some(TermColor::Blue))
            .set_intense(true);

        theme
    }
}
