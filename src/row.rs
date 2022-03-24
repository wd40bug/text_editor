use std::ops::RangeInclusive;

use regex::Regex;
use termion::color::{Fg, Rgb};

use crate::{file_type::HighlightingOptions, highlight::Type};
#[derive(Debug)]
pub struct Row {
    pub content: Vec<String>,
    pub highlighting: Vec<Type>,
}
impl Row {
    #[must_use]
    pub fn to_string(&self, range: RangeInclusive<usize>) -> String {
        let mut result = String::from(&format!("{}", Fg(Rgb(255, 255, 255))));
        let mut last = Type::None;
        for gr in range {
            if self.highlighting[gr] == last {
                result += &self.content[gr].clone();
            } else {
                let content = self.content[gr].clone();
                let output = format!("{}{}", Fg(self.highlighting[gr].highlight()), content,);
                result += &output;
                last = self.highlighting[gr].clone();
            }
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
    #[allow(clippy::must_use_candidate)]
    pub fn search(&self, string: &str) -> Option<usize> {
        let bit_buffer = self.inner_string();
        bit_buffer.find(&string)
    }
    #[allow(clippy::needless_continue)]
    pub fn highlight(
        &mut self,
        word: &Option<String>,
        hilight_ops: &HighlightingOptions,
        in_comment: &mut bool,
    ) {
        let inner_string = self.inner_string();
        self.highlighting = Vec::new();
        for (i, gr) in self.content.iter().enumerate() {
            if self.highlighting.get(i).is_some() {
                continue;
            } else if *in_comment {
                self.highlighting.push(Type::Comment);
                if gr == "*" && self.content.get(i + 1) == Some(&"/".to_string()) {
                    self.highlighting.push(Type::Comment);
                    *in_comment = false;
                }
            } else if hilight_ops.numbers && self.is_used_as_num(i) {
                self.highlighting.push(Type::Number);
            } else if gr == "\"" {
                self.highlighting.push(Type::String);
                for j in i + 1..self.content.len() {
                    self.highlighting.push(Type::String);

                    if self.content[j] == "\"" {
                        break;
                    }
                }
            } else if hilight_ops.characters
                && gr == "'"
                && self.content.get(i + 1) == Some(&"'".to_string())
            {
                for _ in 0..2 {
                    self.highlighting.push(Type::String);
                }
            } else if hilight_ops.characters
                && gr == "'"
                && self.content.get(i + 1) == Some(&"\\".to_string())
            {
                self.highlighting.push(Type::String);
                for j in i + 1..self.content.len() {
                    self.highlighting.push(Type::String);
                    if self.content[j] == "'" {
                        break;
                    }
                }
            } else if hilight_ops.characters
                && gr == "'"
                && self.content.get(i + 2) == Some(&"'".to_string())
            {
                for _ in 0..3 {
                    self.highlighting.push(Type::String);
                }
            } else if hilight_ops.comment
                && gr == "/"
                && self.content.get(i + 1) == Some(&"/".to_string())
            {
                for _ in i..self.content.len() {
                    self.highlighting.push(Type::Comment);
                }
            } else if gr == "/" && self.content.get(i + 1) == Some(&"*".to_string()) {
                self.highlighting.push(Type::Comment);
                *in_comment = true;
            } else {
                self.highlighting.push(Type::None);
            }
        }
        let inner_words = self.get_inner_words();
        let mut i = 0;
        'word: for word in inner_words {
            if hilight_ops.key_words.contains(&word.to_string()) {
                for j in 0..word.len() {
                    if self.highlighting[j + i] != Type::None {
                        continue 'word;
                    }
                }
                for j in 0..word.len() {
                    self.highlighting[j + i] = Type::Keyword;
                }
            } else if hilight_ops.types.contains(&word.to_string()) {
                for j in 0..word.len() {
                    if self.highlighting[j + i] != Type::None {
                        continue 'word;
                    }
                }
                for j in 0..word.len() {
                    self.highlighting[i + j] = Type::Types;
                }
            }
            i += word.len() + 1;
        }
        if let Some(query) = word {
            if let Some(index) = inner_string.find(query) {
                for i in index..query.len() + index {
                    self.highlighting[i] = Type::Match;
                }
            }
        }
    }
    pub fn get_inner_words(&mut self) -> Vec<String> {
        Regex::new(r"[ {};.?\\/+=\-!@#$%^&*():><,.~`|]") //r"((?!\_)[[:punct:]]|\s)"
            .unwrap()
            .split(&self.inner_string())
            .map(|s| s.to_string())
            .collect()
    }
    fn inner_string(&self) -> String {
        let mut bit_buffer = String::new();
        for gr in &self.content {
            bit_buffer += gr;
        }
        bit_buffer
    }
    fn is_used_as_num(&self, index: usize) -> bool {
        if self.content[index] == "."
            && index > 0
            && self.highlighting[index - 1] == Type::Number
            && self.content[index + 1].parse::<i32>().is_ok()
        {
            return true;
        }
        if self.content[index].parse::<isize>().is_err() {
            return false;
        }
        if index == 0 {
            return true;
        }
        for j in (0..index).rev() {
            let ch_array: Vec<char> = self.content[j].chars().collect();
            if self.highlighting[j] == Type::Number {
                return true;
            } else if ch_array.len() == 1 {
                return ch_array[0].is_ascii_punctuation() || ch_array[0].is_ascii_whitespace();
            }
        }
        false
    }
}
