mod buffer;
mod output;
mod global_vars;
mod reader;
mod cursor_controller;
mod editor;
mod rows;

use editor::{Editor, CleanUp};
use std::io::stdout;
use crossterm::{cursor, execute};


fn main() -> crossterm::Result<()> {
    let _cleanup = CleanUp;
    let mut editor = Editor::new();
    while editor.execute()? {};

    println!("Shouldn't print if clear screen works\n\r");
    execute!(stdout(), cursor::MoveTo(0, 0))
}
