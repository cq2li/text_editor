use crossterm::event::{KeyCode};
use crate::rows::EditorRows;
use std::cmp::{min, max};
use crate::rows::{Row, TAB_STOP};

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    size_x: usize,
    size_y: usize,
    pub row_offset: usize,
    pub col_offset: usize,
    pub render_x: usize,
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
            render_x: 0, // the actual cursor when accounting for tabs
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let y_lim = editor_rows.num_rows();
        let x_lim = if self.cursor_y >= y_lim {
            0
        } else {
            editor_rows.get_render(self.cursor_y).len()
        };
        let x_lim_above = if self.cursor_y == 0 {
            0
        } else {
            editor_rows.get_row(self.cursor_y - 1).row_content.len()
        };
        match direction {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor_y != 0 {
                    self.cursor_x = min(self.cursor_x, x_lim_above);
                }
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
                if self.cursor_x == 0 && self.cursor_y != 0 {
                    self.cursor_x = x_lim_above;
                    self.cursor_y = self.cursor_y - 1;
                } else if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Right | KeyCode::Char('l') => {
                if self.render_x == x_lim {
                    self.cursor_x = 0;
                    self.cursor_y = min(y_lim, self.cursor_y + 1);
                } else if self.cursor_x < x_lim {
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
            editor_rows.get_render(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = min(self.cursor_x, row_len);
    }
    

    /* @brief adjusts column and row offsets so that the cursor in the buffer
     *        is always rendered in the terminal
     */
    pub fn scroll(&mut self, editor_rows: &EditorRows) {
        // update render_x which is the position of the cursor on terminal
        //  differs from cursor_x in the buffer due to tabs being expanded to 
        //  4 spaces
        (self.render_x, self.cursor_x) = 
            if self.cursor_y < editor_rows.num_rows() {
                (self.get_render_x(&editor_rows.get_row(self.cursor_y)),
                min(editor_rows.get_row(self.cursor_y).row_content.len(), 
                    self.cursor_x))
            } else {
                (0, 0)
            };
        // row_offset is the position where scree rendering starts
        //  next 2 blocks ensures cursor_y is always on screen
        if self.cursor_y >= self.row_offset + self.size_y {
            self.row_offset = self.cursor_y - self.size_y + 1;
        }
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }
        // cursor x is always on screen
        if self.render_x >= self.col_offset + self.size_x {
            self.col_offset = self.render_x - self.size_x + 1;
        }
        if self.render_x < self.col_offset {
            self.col_offset = self.render_x;
        }


    }

    pub fn get_render_x(&mut self, row: &Row) -> usize {
        let cursor_x = min(row.row_content.len(), self.cursor_x);
        row.row_content[..cursor_x]
            .chars()
            .fold(0, |accm, chr| {
                if chr == '\t' {
                    // move to tab stp to left and hop right one tab
                    accm - (accm % TAB_STOP) + TAB_STOP
                } else {
                    accm + 1
                }
            })
    }
}
