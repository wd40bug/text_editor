use std::path::PathBuf;

pub struct FileType {
    pub name: String,
    pub highlight_ops: HighlightingOptions,
}
#[derive(Default, Clone)]
pub struct HighlightingOptions {
    pub numbers: bool,
    pub strings: bool,
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
