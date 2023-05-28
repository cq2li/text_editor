use crossterm::event::{KeyCode};
use crossterm::terminal;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    size_x: usize,
    size_y: usize,
}

impl CursorController {
    pub fn new((size_x, size_y): (usize, usize)) -> Self {
        Self {
            cursor_x: 0, 
            cursor_y: 0,
            size_x,
            size_y,
              
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            },
            KeyCode::Down => {
                if self.cursor_y < self.size_y - 1 {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Right => {
                if self.cursor_x < self.size_x - 1 {
                    self.cursor_x += 1;
                }
            },
            _ => unimplemented!(),

        }
    }
} 
