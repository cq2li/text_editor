use crossterm::event::{KeyCode};
use std::cmp;

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
        // let v_offset_lim = v_offset_lim - self.size_y + 1;
        let h_offset_lim = h_offset_lim.saturating_sub(self.size_x - 1);
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
                // possibly update offset in buffer display
                if self.cursor_y == self.size_y - 1 && self.row_offset < v_offset_lim {
                    self.row_offset += 1;
                }
                // realign to end of buffer if current over hangs next line
                if h_offset_lim == 0 {
                    //pass
                } else if self.col_offset > h_offset_lim - 1 {
                    self.col_offset = h_offset_lim - 1;
                }
                if self.cursor_y < self.size_y - 1 {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Left | KeyCode::Char('h') => {
                if self.cursor_x == 0 && self.col_offset > 0 {
                    self.col_offset -= 1;
                }
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Right | KeyCode::Char('l') => {
                if self.cursor_x == self.size_x - 1 && self.col_offset < h_offset_lim.saturating_sub(1) {
                    self.col_offset += 1;
                }
                if self.cursor_x < self.size_x - 1 {
                    self.cursor_x += 1;
                }
            },
            KeyCode::End => {
                self.cursor_x = self.size_x - 1;
                self.col_offset = h_offset_lim.saturating_sub(1);

            },
            KeyCode::Home => {
                self.cursor_x = 0;
                self.col_offset = 0;
            }
            _ => unimplemented!(),
        }
    }

    pub fn scroll(&mut self) {
        //TODO
        ()
    }
} 
