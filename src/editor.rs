use crossterm::{event, terminal};
use event::{Event, KeyCode, KeyModifiers};

use crate::output::Output;
use crate::prompt;
use crate::reader::Reader;

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
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_keyevent(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_keyevent() {
            /* exit the program */
            Some(event::KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                if self.output.dirty > 0 {
                    self.output.status_message.set_message(
                        "Modified! CTRL + q to exit or ESC to cancel"
                            .to_string(),
                    );
                    self.output.refresh()?;
                    match event::read().unwrap() {
                        Event::Key(event::KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: event::KeyModifiers::CONTROL,
                            ..
                        }) => return Ok(false),
                        _ => return Ok(true),
                    }
                }
                return Ok(false);
            }
            /* movement controller */
            Some(event::KeyEvent {
                code:
                    direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End
                    | KeyCode::PageUp
                    | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.move_cursor(direction),
            Some(event::KeyEvent {
                code:
                    direction @ (KeyCode::Char('h')
                    | KeyCode::Char('j')
                    | KeyCode::Char('k')
                    | KeyCode::Char('l')),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => self.output.move_cursor(direction),
            /* saving document */
            Some(event::KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                if matches!(self.output.editor_rows.filename, None) {
                    self.output.editor_rows.filename =
                        prompt!(&mut self.output, "Save as : {}").map(|it| it.into());
                    if let None = self.output.editor_rows.filename {
                        return Ok(true);
                    }
                }
                let len_written = self.output.save()?;
                self.output
                    .status_message
                    .set_message(format!("{} bytes written to disk", len_written))
            }
            /* deletions */
            Some(event::KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.backspace(),
            Some(event::KeyEvent {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.del(),
            /* new line */
            Some(event::KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            }) => self.output.enter(),
            /* editing document content */
            Some(event::KeyEvent {
                code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                modifiers: case @ (KeyModifiers::NONE | KeyModifiers::SHIFT),
                ..
            }) => self.output.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(char) if matches!(case, KeyModifiers::NONE) => char,
                KeyCode::Char(char) if matches!(case, KeyModifiers::SHIFT) => {
                    char.to_ascii_uppercase()
                }
                _ => unimplemented!(),
            }),
            None => (),
            _ => (),
        }
        return Ok(true);
    }

    pub fn execute(&mut self) -> crossterm::Result<bool> {
        loop {
            self.output.refresh()?;
            if !self.process_keyevent()? {
                return Ok(true);
            }
        }
    }
}
