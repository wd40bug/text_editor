pub struct FileType {
    pub name: String,
    pub highlight_ops: HighlightingOptions,
}
#[derive(Default)]
pub struct HighlightingOptions {
    pub numbers: bool,
}
impl Default for FileType {
    fn default() -> Self {
        FileType {
            name: String::from("No Filetype"),
            highlight_ops: (HighlightingOptions::default()),
        }
    }
}
