use crate::buffer::Buf;
use crate::global_vars::VERSION;

use std::io::{stdout, Write};

use crossterm::{cursor, terminal, execute, queue};
use terminal::{ClearType};

pub struct Output {
    size: (u16, u16),
    buffer: Buf, 
}

impl Output {
    pub fn new() -> Self {
        Self { size: terminal::size().unwrap(),
               buffer: Buf::new(),
        }
    }

    pub fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
    
    pub fn draw_rows(&mut self) {
        let display_x = self.size.0;
        let display_y = self.size.1;
        let line_marker = "💩";
        for i in 0..display_y {
            self.buffer.push_str(line_marker);
            
            if i == display_y / 10 {
                let mut welcome = format!("Poop Editor --- Version {}", VERSION);
                if welcome.len() > display_x as usize {
                    welcome.truncate(display_x as usize);
                }
                let padding = (display_x as usize - welcome.len()) / 2;
                (line_marker.len()..padding).for_each(|_| self.buffer.push_chr(' '));
                self.buffer.push_str(&welcome);
            }

            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < display_y - 1 {
                self.buffer.push_str("\n\r");
            }
        }
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.clear_screen()?;
        queue!(self.buffer, cursor::MoveTo(0,0))?;
        self.draw_rows();
        queue!(self.buffer, cursor::MoveTo(0,0))?;
        self.buffer.flush()
    }
}
