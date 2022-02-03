use super::{
    style::{Color, Style},
    HeaderTheme, RequestTheme, ResponseTheme, Theme,
};

#[cfg_attr(test, derive(Debug))]
#[derive(Clone, Copy)]
pub struct DefaultTheme {}
#[derive(Clone, Copy)]
pub struct DefaultReponseTheme {}
#[derive(Clone, Copy)]
pub struct DefaultRequestTheme {}

impl RequestTheme for DefaultRequestTheme {
    fn as_header(&self) -> &dyn HeaderTheme {
        self
    }
    fn primary(&self) -> Style {
        Color::Purple.normal()
    }
    fn secondary(&self) -> Style {
        Color::Purple.normal()
    }
    fn url(&self) -> Style {
        Color::Purple.normal()
    }
    fn direction(&self, standard: bool) -> Style {
        crate::ifelse!(standard, self.primary(), self.secondary())
    }
}
impl HeaderTheme for DefaultRequestTheme {
    fn header_name(&self, standard: bool) -> Style {
        crate::ifelse!(standard, self.primary(), self.secondary())
    }
    fn header_value(&self, _: bool) -> Style {
        Style::default()
    }
}

impl ResponseTheme for DefaultReponseTheme {
    fn as_header(&self) -> &dyn HeaderTheme {
        self
    }
    fn primary(&self) -> Style {
        Color::Green.normal()
    }
    fn secondary(&self) -> Style {
        Color::Cyan.normal()
    }
    fn version(&self) -> Style {
        Color::Green.normal()
    }
    fn status(&self) -> Style {
        Color::Green.bold()
    }
    fn direction(&self, standard: bool) -> Style {
        crate::ifelse!(standard, self.primary(), self.secondary())
    }
}
impl HeaderTheme for DefaultReponseTheme {
    fn header_name(&self, standard: bool) -> Style {
        crate::ifelse!(standard, self.primary(), self.secondary())
    }
    fn header_value(&self, _: bool) -> Style {
        Style::default()
    }
}

impl Theme for DefaultTheme {
    fn request(&self) -> Box<dyn RequestTheme> {
        Box::new(DefaultRequestTheme {})
    }
    fn response(&self) -> Box<dyn ResponseTheme> {
        Box::new(DefaultReponseTheme {})
    }
}

impl DefaultTheme {
    pub fn new() -> DefaultTheme {
        DefaultTheme {}
    }
}
