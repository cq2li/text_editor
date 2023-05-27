mod buffer;

use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::cursor;
use crossterm::{event, terminal, terminal::ClearType, execute};
use event::{Event, KeyCode};

use buffer::Buf;


struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not enable raw mode");
        execute!(stdout(), cursor::Hide).expect("Could not hide cursor");
        Self { reader: Reader,
               output: Output::new(),
        }
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
            event::KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::ALT,
                ..
            } => self.clear_screen().expect("Couldn't clear screen"),
            event@event::KeyEvent { .. } 
              => println!("{:?}\r", event),
        }
        return Ok(true)
        
    }

    fn execute(&self) -> crossterm::Result<bool> {
        self.output.refresh()?;
        self.process_keyevent()
    }

    fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        println!("\x1b[2J"); // clears screen with esc characters
        execute!(stdout(), cursor::Show).expect("Could not show cursor");
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

struct Output {
    size: (u16, u16),
    buffer: Buf, 
}

impl Output {
    fn new() -> Self {
        Self { size: terminal::size().unwrap(),
               buffer: Buf::new(),
        }
    }

    fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
    
    fn draw_rows(&self) {
        for _ in 0..self.size.1-1 {
            println!("💩\r");
        }
        print!("💩\r"); stdout().flush().unwrap(); // last line not \n
    }

    fn refresh(&self) -> crossterm::Result<()> {
        self.clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}

fn main() -> crossterm::Result<()> {
    let editor = Editor::new();
    while editor.execute()? {};

    println!("Shouldn't print if clear screen works\n\r");
    Ok(())
}
