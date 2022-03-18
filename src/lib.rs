pub mod document;
pub mod editor;
pub mod terminal;
#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize,
}
