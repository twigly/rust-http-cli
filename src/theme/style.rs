#[derive(Default)]
pub struct Style {
    pub forecolor: Option<Color>,
    pub backcolor: Option<Color>,
    pub is_bold: bool,
    pub is_dimmed: bool,
    pub newline: bool,
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

impl Style {
    pub fn newline() -> Style {
        Style {
            newline: true,
            ..Default::default()
        }
    }
}

impl Color {
    pub fn normal(self) -> Style {
        Style {
            forecolor: Some(self),
            ..Default::default()
        }
    }
    pub fn normal_newline(self) -> Style {
        Style {
            forecolor: Some(self),
            newline: true,
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
    pub fn bold_newline(self) -> Style {
        Style {
            forecolor: Some(self),
            is_bold: true,
            newline: true,
            ..Default::default()
        }
    }
}
