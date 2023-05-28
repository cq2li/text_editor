use crate::buffer::Buf;
use crate::global_vars::VERSION;
use crate::cursor_controller::CursorController;

use std::io::{stdout, Write};

use crossterm::{cursor, terminal, execute, queue, event};
use terminal::{ClearType};

pub struct Output {
    size: (usize, usize),
    buffer: Buf, 
    cursor_controller: CursorController,
}

impl Output {
    pub fn new() -> Self {
        let size = terminal::size().map(|(x, y)| (x as usize, y as usize)).unwrap();
        Self { size,
               buffer: Buf::new(),
               cursor_controller: CursorController::new(size),
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
        queue!(self.buffer, cursor::MoveTo(self.cursor_controller.cursor_x as u16, self.cursor_controller.cursor_y as u16))?;
        self.buffer.flush()
    }

    pub fn move_cursor(&mut self, direction: event::KeyCode) {
        self.cursor_controller.move_cursor(direction)
    }
}
