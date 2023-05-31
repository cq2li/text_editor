use std::io::{stdout, Write};

use crossterm::{cursor, event, terminal, execute, queue};
use event::{Event, KeyCode};
use terminal::{ClearType};

use crate::reader::Reader;
use crate::output::Output;

// @brief stores clean up code in drop
pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        // println!("\x1b[2J"); // clears screen with esc characters
        if terminal::is_raw_mode_enabled().unwrap() {
            terminal::disable_raw_mode().expect("Couldn't disable raw mode");
        }
    }
    
}

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not enable raw mode");
        // execute!(stdout(), cursor::Hide).expect("Could not hide cursor");
        Self { reader: Reader,
               output: Output::new(),
        }
    }

    fn process_keyevent(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_keyevent()? {
            event::KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            event::KeyEvent {
                code: direction @ (KeyCode::Up|KeyCode::Down|
                                   KeyCode::Char('j')|KeyCode::Char('k')|
                                   KeyCode::Left|KeyCode::Right|
                                   KeyCode::Char('h')|KeyCode::Char('l')),
                modifiers: event::KeyModifiers::NONE,
                ..
            } => self.output.move_cursor(direction),
            event@event::KeyEvent { .. } 
              => println!("{:?}\r", event),
        }
        return Ok(true)
        
    }

    pub fn execute(&mut self) -> crossterm::Result<bool> {
        self.output.refresh()?;
        self.process_keyevent()
    }

    fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))
    } 
}
