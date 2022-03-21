use termion::color::Rgb;

#[derive(Debug)]
pub enum Type {
    None,
    Number,
}
impl Type {
    pub fn highlight(&self) -> Rgb {
        match self {
            Self::Number => return Rgb(103, 18, 107),
            Self::None => return Rgb(255, 255, 255),
        }
    }
}
