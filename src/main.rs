mod buffer;
mod output;
mod global_vars;
mod reader;
mod editor;

use editor::Editor;
use std::io::stdout;
use crossterm::{cursor, execute};


fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new();
    while editor.execute()? {};

    println!("Shouldn't print if clear screen works\n\r");
    execute!(stdout(), cursor::MoveTo(0, 0))
}
