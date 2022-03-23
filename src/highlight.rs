use termion::color::Rgb;

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Comment,
    Keyword,
    Types,
}
impl Type {
    #[allow(clippy::must_use_candidate)]
    pub fn highlight(&self) -> Rgb {
        match self {
            Self::Number => Rgb(103, 18, 107),
            Self::None => Rgb(255, 255, 255),
            Self::Match => Rgb(12, 145, 194),
            Self::String => Rgb(17, 209, 55),
            Self::Comment => Rgb(135, 129, 128),
            Self::Keyword => Rgb(85, 0, 255),
            Self::Types => Rgb(196, 194, 59),
        }
    }
}
