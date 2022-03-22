use std::time::Instant;

pub mod document;
pub mod editor;
pub mod file_type;
pub mod highlight;
pub mod row;
pub mod terminal;
#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize,
}
pub struct StatusMessage {
    pub message: String,
    pub time: Instant,
}
impl StatusMessage {
    #[must_use]
    pub fn new(message: String) -> StatusMessage {
        StatusMessage {
            message,
            time: Instant::now(),
        }
    }
}
