use crate::theme::style::Color;
use termcolor::Color as TermColor;

impl From<Color> for TermColor {
    fn from(color: Color) -> TermColor {
        match color {
            Color::Black => TermColor::Black,
            Color::Red => TermColor::Red,
            Color::Green => TermColor::Green,
            Color::Yellow => TermColor::Yellow,
            Color::Blue => TermColor::Blue,
            Color::Purple => TermColor::Magenta,
            Color::Cyan => TermColor::Cyan,
            Color::White => TermColor::White,
        }
    }
}
