use crossterm::event::{KeyCode};
use crate::output;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    size_x: usize,
    size_y: usize,
    pub row_offset: usize, // track file row position
    pub col_offset: usize, // track file col position
}

impl CursorController {
    pub fn new((size_x, size_y): (usize, usize)) -> Self {
        Self {
            cursor_x: 0, 
            cursor_y: 0,
            size_x,
            size_y,
            row_offset: 0,
            col_offset: 0,
              
        }
    }

    pub fn move_cursor(&mut self, 
                       direction: KeyCode, 
                       v_offset_lim: usize,
                       h_offset_lim: usize) {
        match direction {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor_y == 0 && self.row_offset > 0 {
                    self.row_offset -= 1;
                }
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            },
            KeyCode::Down | KeyCode::Char('j') => {
                if self.cursor_y == self.size_y - 1 && self.row_offset < v_offset_lim {
                    self.row_offset += 1;
                }
                if self.cursor_y < self.size_y - 1 {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Left | KeyCode::Char('h') => {
                if self.cursor_y == 0 && self.col_offset > 0 {
                    self.col_offset -= 1;
                }
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Right | KeyCode::Char('l')=> {
                
                if self.cursor_x == self.size_x - 1 && self.col_offset < h_offset_lim - 1 {
                    self.col_offset += 1;
                }
                if self.cursor_x < self.size_x - 1 {
                    self.cursor_x += 1;
                }
            },
            _ => unimplemented!(),
        }
    }

    pub fn scroll(&mut self) {
        //TODO
        ()
    }
} 
