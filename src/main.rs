use io::Read;
use std::io;
use std::time::Duration;

use crossterm::{event, terminal};
use event::{Event, KeyCode};

struct Editor {
    reader: Reader,
}

impl Editor {
    fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not enable raw mode");
        Self { reader: Reader }
    }

    fn close(&self) -> crossterm::Result<()> {
        if terminal::is_raw_mode_enabled()? {
            terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    fn process_keyevent(&self) -> crossterm::Result<bool> {
        match self.reader.read_keyevent()? {
            event::KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),
            event@event::KeyEvent {
                code: KeyCode::Char(_),
                ..
            } => println!("{:?}\r", event),
            _ => {
                println!("No Input\r");
            }
        }
        return Ok(true)
        
    }

    fn execute(&self) -> crossterm::Result<bool> {
        self.process_keyevent()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.close().expect("Could not disable raw mode");
    }
}

struct Reader;

impl Reader {
    // Reads one byte of input
    // times out and does not block
    fn read_keyevent(&self) -> crossterm::Result<event::KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

fn main() -> crossterm::Result<()> {
    let _e = Editor::new();
    let mut stdin = io::stdin();
    let ref_stdin = io::BufReader::new(&mut stdin);

    for byte in ref_stdin.bytes() {
        let char = byte.unwrap() as char;
        if char.is_control() {
            println!("{}\r", char as u8);
        } else {
            println!("{}\r", char);
        }
        if char == 'q' {
            break;
        }
    }

    for byte in (&mut stdin).bytes() {
        println!("{}\r", byte.as_ref().unwrap());
        if byte.unwrap() == b'w' {
            break;
        }
    }

    for byte in stdin.bytes() {
        println!("{}\r", byte.as_ref().unwrap());
        if byte.unwrap() == b'e' {
            break;
        }
    }

    'ct_read: loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    event::KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => break 'ct_read,
                    event::KeyEvent {
                        code: KeyCode::Char(chr),
                        ..
                    } => {
                        println!("{}\r", chr);
                    }
                    _ => {
                        println!("NOPE\r");
                    }
                }
                println!("{:?}\r", event);
            }
        } else {
            println!("No input\r");
        }
    }

    println!("Hello, world!\n\r");
    Ok(())
}
