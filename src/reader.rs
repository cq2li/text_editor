use std::time::Duration;

use crossterm::event;
use event::Event;


pub struct Reader;

impl Reader {
    // Reads one byte of input
    // times out and does not block
    pub fn read_keyevent(&self) -> Option<event::KeyEvent> {
            if event::poll(Duration::from_millis(16)).unwrap() {
                if let Event::Key(event) = event::read().unwrap() {
                    return Some(event);
                }
            }
            return None
    }
}

