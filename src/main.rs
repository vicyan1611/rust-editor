mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    match editor.run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("There is a error when running editor, {}", e)
        }
    }
}
