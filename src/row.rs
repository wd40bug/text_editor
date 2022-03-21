use std::ops::RangeInclusive;

use termion::color::{Fg, Reset};

use crate::highlight::Type;
#[derive(Debug)]
pub struct Row {
    pub content: Vec<String>,
    pub highlighting: Vec<Type>,
}
impl Row {
    #[must_use]
    pub fn to_string(&self, range: RangeInclusive<usize>) -> String {
        let mut result = String::new();
        for gr in range {
            let mut output = String::new();
            let content = self.content[gr].clone();
            if content.parse::<isize>().is_ok() {
                output += &format!(
                    "{}{}{}",
                    Fg(self.highlighting[gr].highlight()),
                    content,
                    Fg(Reset)
                );
            } else {
                output = content;
            }
            result += &output;
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
    pub fn search(&self, string: &str) -> Option<usize> {
        let mut bit_buffer = String::new();
        for gr in &self.content {
            bit_buffer += gr;
        }
        bit_buffer.find(&string)
    }
    pub fn highlight(&mut self) {
        self.highlighting = Vec::new();
        for gr in &self.content {
            if gr.parse::<isize>().is_ok() {
                self.highlighting.push(Type::Number);
            } else {
                self.highlighting.push(Type::None);
            }
        }
    }
}
