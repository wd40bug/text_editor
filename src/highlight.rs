use termion::color::Rgb;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    None,
    Number,
    Match,
}
impl Type {
    pub fn highlight(&self) -> Rgb {
        match self {
            Self::Number => return Rgb(103, 18, 107),
            Self::None => return Rgb(255, 255, 255),
            Self::Match => return Rgb(12, 145, 194),
        }
    }
}
