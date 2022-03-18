use text_editor::{editor::Editor, document::Document};
use clap::Parser;

fn main() {
    simple_logging::log_to_file("last log.log", log::LevelFilter::Debug).unwrap();
    let args = Args::parse();
    let document = Document::new(args.path);
    let mut editor = Editor::new(document);
    editor.run();
}
#[derive(Parser)]
struct Args{
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
