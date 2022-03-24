use std::{path::PathBuf, vec};

pub struct FileType {
    pub name: String,
    pub highlight_ops: HighlightingOptions,
}
#[derive(Default, Clone)]
pub struct HighlightingOptions {
    pub numbers: bool,
    pub strings: bool,
    pub comment: bool,
    pub characters: bool,
    pub multi_comment: bool,
    //NEITHER KEY_WORDS NOR TYPES MAY CONTAIN MULTI BYTE UNICODE!!!
    pub key_words: Vec<String>,
    pub types: Vec<String>,
}
impl Default for FileType {
    fn default() -> Self {
        FileType {
            name: String::from("No Filetype"),
            highlight_ops: (HighlightingOptions::default()),
        }
    }
}
impl From<PathBuf> for FileType {
    fn from(buf: PathBuf) -> Self {
        return if let Some(ext) = buf.extension() {
            if let Some(string) = ext.to_str() {
                match string {
                    "rs" => FileType {
                        name: "Rust".to_string(),
                        highlight_ops: HighlightingOptions {
                            numbers: true,
                            strings: true,
                            comment: true,
                            characters: true,
                            multi_comment: true,
                            key_words: vec![
                                "use".to_string(),
                                "fn".to_string(),
                                "let".to_string(),
                                "mut".to_string(),
                                "impl".to_string(),
                                "for".to_string(),
                                "in".to_string(),
                                "type".to_string(),
                                "move".to_string(),
                                "if".to_string(),
                                "else".to_string(),
                                "pub".to_string(),
                                "break".to_string(),
                                "const".to_string(),
                                "continue".to_string(),
                                "crate".to_string(),
                                "enum".to_string(),
                                "extern".to_string(),
                                "loop".to_string(),
                                "match".to_string(),
                                "mod".to_string(),
                                "ref".to_string(),
                                "return".to_string(),
                                "static".to_string(),
                                "struct".to_string(),
                                "super".to_string(),
                                "trait".to_string(),
                                "unsafe".to_string(),
                                "where".to_string(),
                                "while".to_string(),
                            ],
                            types: vec![
                                "Self".to_string(),
                                "true".to_string(),
                                "false".to_string(),
                                "u16".to_string(),
                                "usize".to_string(),
                                "u32".to_string(),
                                "u64".to_string(),
                                "u128".to_string(),
                                "i16".to_string(),
                                "u8".to_string(),
                                "i8".to_string(),
                                "i32".to_string(),
                                "i64".to_string(),
                                "i128".to_string(),
                                "String".to_string(),
                                "bool".to_string(),
                                "char".to_string(),
                                "isize".to_string(),
                            ],
                        },
                    },
                    _ => Self::default(),
                }
            } else {
                Self::default()
            }
        } else {
            Self::default()
        };
    }
}
