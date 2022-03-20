use clap::Parser;
use text_editor::{document::Document, editor::Editor};

fn main() {
    simple_logging::log_to_file("last log.log", log::LevelFilter::Debug).unwrap();
    let args = Args::parse();
    let document = Document::new(args.path);
    let mut editor = Editor::new(document);
    editor.run();
}
#[derive(Parser)]
struct Args {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
