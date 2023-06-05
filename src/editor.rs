use std::io::{stdout, Write};

use crossterm::{cursor, event, terminal, execute, queue};
use event::{Event, KeyCode, KeyModifiers};
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
        match self.reader.read_keyevent() {
            Some(event::KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => return Ok(false),
            Some(event::KeyEvent {
                code: direction @ 
                    (KeyCode::Up|KeyCode::Down|
                     KeyCode::Left|KeyCode::Right|
                     KeyCode::Home|KeyCode::End|
                     KeyCode::PageUp|KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.move_cursor(direction),
            Some(event::KeyEvent {
                code: direction @ 
                    (KeyCode::Char('h')|KeyCode::Char('j')|
                     KeyCode::Char('k')|KeyCode::Char('l')),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => self.output.move_cursor(direction),
            Some(event::KeyEvent {
                code: code @ (KeyCode::Backspace|KeyCode::Delete),
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.delete_char(code),
            Some(event::KeyEvent {
                code: code @ (KeyCode::Char(..)|KeyCode::Tab),
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(char) => char,
                _ => unimplemented!(),
            }),
            None => (),
            _ => (),
        }
        return Ok(true)
        
    }

    pub fn execute(&mut self) -> crossterm::Result<bool> {
        loop {
            self.output.refresh()?;
            if !self.process_keyevent()? {
                return Ok(true)
            }
        }
    }
}
