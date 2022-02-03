#[derive(Default)]
pub struct Style {
    pub forecolor: Option<Color>,
    pub backcolor: Option<Color>,
    pub is_bold: bool,
    pub is_dimmed: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

impl Color {
    pub fn normal(self) -> Style {
        Style {
            forecolor: Some(self),
            ..Default::default()
        }
    }
    pub fn bold(self) -> Style {
        Style {
            forecolor: Some(self),
            is_bold: true,
            ..Default::default()
        }
    }
}
