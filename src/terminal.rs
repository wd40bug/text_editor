pub struct Terminal {
    pub width: u16,
    pub height: u16,
}
impl Terminal {
    #[must_use]
    pub fn new(tuple: (u16, u16)) -> Terminal {
        Terminal {
            width: tuple.0,
            height: tuple.1,
        }
    }
}
