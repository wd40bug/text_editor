use std::{fs::read_to_string, io::Write, ops::RangeInclusive, path::PathBuf};

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
    pub path: Option<PathBuf>,
}
impl Document {
    ///# Panics
    ///
    /// panics if file isn't there
    #[must_use]
    pub fn new(path: Option<PathBuf>) -> Document {
        let mut rows = Vec::new();
        if let Some(path) = path.clone() {
            let content = read_to_string(&path).unwrap();
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
        } else {
            rows = vec![Row {
                content: Vec::new(),
            }];
        }
        Document { rows, path }
    }
    pub fn save(&self) {
        let path = self.path.clone().unwrap();
        let mut file = std::fs::File::create(path).unwrap();
        for row in &self.rows {
            let mut foo = String::new();
            for gr in &row.content {
                foo.push_str(&gr);
            }
            foo.push('\n');
            file.write_all(foo.as_bytes()).unwrap();
        }
    }
    pub fn save_as(&mut self, path: String) {
        let mut file = std::fs::File::create(path.clone()).unwrap();
        for row in &self.rows {
            let mut foo = String::new();
            for gr in &row.content {
                foo.push_str(&gr);
            }
            foo.push('\n');
            file.write_all(foo.as_bytes()).unwrap();
        }
        self.path = Some(PathBuf::from(path));
    }
}
