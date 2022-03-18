use std::{fs::read_to_string, ops::RangeInclusive, path::PathBuf};

use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub struct Row {
    pub content: Vec<String>,
}
impl Row {
    pub fn to_string(&self, range: RangeInclusive<usize>) -> String {
        let mut result = String::new();
        for gr in range {
            result += &self.content[gr];
        }
        result
    }
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
                content: line
                    .graphemes(true)
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|str| str.to_string())
                    .collect(),
            });
            log::info!("{:?}", rows.last().unwrap());
        }
        Document { rows, path }
    }
}
