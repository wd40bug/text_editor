use std::{fs::read_to_string, io::Write, path::PathBuf};

use unicode_segmentation::UnicodeSegmentation;

use crate::{file_type::FileType, row::Row, Position};

pub struct Document {
    pub rows: Vec<Row>,
    pub path: Option<PathBuf>,
    pub file_type: FileType,
}
impl Document {
    pub fn highlight(&mut self, word: &Option<String>) {
        for row in &mut self.rows {
            row.highlight(word, self.file_type.highlight_ops.clone());
        }
    }
    ///# Panics
    ///
    /// panics if file isn't there
    #[must_use]
    pub fn new(path: Option<PathBuf>) -> Document {
        let mut rows = Vec::new();
        let file_path;
        if let Some(path) = path.clone() {
            file_path = FileType::from(path.clone());
            let content = read_to_string(&path).unwrap();
            for line in content.lines() {
                rows.push(Row {
                    content: line
                        .graphemes(true)
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|str| (*str).to_string())
                        .collect(),
                    highlighting: Vec::new(),
                });
            }
            log::info!("{}", rows.len());
            for row in &mut rows {
                row.parse_specials();
            }
        } else {
            file_path = FileType::default();
            rows = vec![Row {
                content: Vec::new(),
                highlighting: Vec::new(),
            }];
        }
        Document {
            rows,
            path,
            file_type: file_path,
        }
    }
    ///# Panics
    ///
    /// panics if file creation fails
    pub fn save(&self) {
        let path = self.path.clone().unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        for row in &self.rows {
            let mut bit_buffer = String::new();
            for gr in &row.content {
                bit_buffer.push_str(gr);
            }
            bit_buffer.push('\n');
            file.write_all(bit_buffer.as_bytes()).unwrap();
        }
    }
    ///# Panics
    ///
    /// panics if the file creation fails
    pub fn save_as(&mut self, path: String) {
        let mut file = std::fs::File::create(path.clone()).unwrap();
        for row in &self.rows {
            let mut bit_buf = String::new();
            for gr in &row.content {
                bit_buf.push_str(gr);
            }
            bit_buf.push('\n');
            file.write_all(bit_buf.as_bytes()).unwrap();
        }
        self.path = Some(PathBuf::from(path));
        self.file_type = FileType::from(self.path.clone().unwrap());
    }
    pub fn search(&self, string: String) -> Vec<Position> {
        let mut result = Vec::new();
        for (i, row) in self.rows.iter().enumerate() {
            if let Some(u) = row.search(&string) {
                result.push(Position { x: u, y: i + 1 });
            }
        }
        result
    }
}
