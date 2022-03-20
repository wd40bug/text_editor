use std::{fs::read_to_string, ops::RangeInclusive, path::PathBuf};

use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub struct Row {
    pub content: Vec<String>,
}
impl Row {
    #[must_use]
    pub fn to_string(&self, range: RangeInclusive<usize>) -> String {
        let mut result = String::new();
        for gr in range {
            result += &self.content[gr];
        }
        result
    }
    pub fn parse_specials(&mut self) {
        for (i, gr) in self.content.clone().iter().enumerate() {
            if gr == "\t" {
                self.content.remove(i);
                self.content.insert(i, " ".to_string());
                self.content.insert(i, " ".to_string());
                self.content.insert(i, " ".to_string());
                self.content.insert(i, " ".to_string());
            }
        }
    }
}
pub struct Document {
    pub rows: Vec<Row>,
    pub path: PathBuf,
}
impl Document {
    ///# Panics
    ///
    /// panics if file isn't there
    #[must_use]
    pub fn new(path: PathBuf) -> Document {
        let content = read_to_string(&path).unwrap();
        let mut rows = Vec::new();
        for line in content.lines() {
            rows.push(Row {
                content: line
                    .graphemes(true)
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|str| (*str).to_string())
                    .collect(),
            });
            log::info!("{:?}", rows.last().unwrap());
        }
        log::info!("{}", rows.len());
        for row in &mut rows {
            row.parse_specials();
        }
        Document { rows, path }
    }
}
