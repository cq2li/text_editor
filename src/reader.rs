use std::time::Duration;

use crossterm::event;
use event::Event;


pub struct Reader;

impl Reader {
    // Reads one byte of input
    // times out and does not block
    pub fn read_keyevent(&self) -> crossterm::Result<event::KeyEvent> {

        loop {
            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

