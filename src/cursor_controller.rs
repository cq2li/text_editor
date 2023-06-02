use crossterm::event::{KeyCode};
use crate::rows::EditorRows;
use std::cmp::{min, max};

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    size_x: usize,
    size_y: usize,
    pub row_offset: usize,
    pub col_offset: usize,
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

    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows:&EditorRows) {
        let y_lim = editor_rows.num_rows();
        let x_lim = editor_rows.get_row(min(self.cursor_y, y_lim-1)).len();
        match direction {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            },
            KeyCode::Down | KeyCode::Char('j') => {
                if self.cursor_y < y_lim {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Left | KeyCode::Char('h') => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Right | KeyCode::Char('l') => {
                if self.cursor_x < x_lim {
                    self.cursor_x += 1;
                }
            },
            KeyCode::End => {
                self.cursor_x = x_lim;
            },
            KeyCode::Home => {
                self.cursor_x = 0;
            }
            KeyCode::PageUp => {
                self.cursor_y = self.cursor_y.saturating_sub(self.size_y);
            },
            KeyCode::PageDown => {
                self.cursor_y = min(y_lim, self.cursor_y + self.size_y);
            }
            _ => unimplemented!(),
        }

        let row_len = if self.cursor_y < y_lim {
            editor_rows.get_row(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = min(self.cursor_x, row_len);
    }

    pub fn scroll(&mut self) {
        if self.cursor_y >= self.row_offset + self.size_y {
            self.row_offset = self.cursor_y - self.size_y + 1;
        }
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }
        if self.cursor_x >= self.col_offset + self.size_x {
            self.col_offset = self.cursor_x - self.size_x + 1;
        }
        if self.cursor_x < self.col_offset {
            self.col_offset = self.cursor_x;
        }
    }
}
