use std::{fs::read_to_string, path::PathBuf};
#[derive(Debug)]
pub struct Row {
    pub content: String,
}
pub struct Document {
    pub rows: Vec<Row>,
    pub path: PathBuf,
}
impl Document {
    pub fn new(path: PathBuf) -> Document {
        let content = read_to_string(&path).unwrap();
        let mut rows = Vec::new();
        for line in content.lines() {
            rows.push(Row {
                content: line.to_string(),
            });
        }
        Document { rows, path }
    }
}
