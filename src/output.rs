use crate::buffer::Buf;
use crate::cursor_controller::CursorController;
use crate::global_vars::VERSION;
use crate::rows::EditorRows;

use std::cmp::{min, max};
use std::io::{stdout, Write};

use crossterm::{cursor, event, execute, queue, terminal};
use terminal::ClearType;

pub struct Output {
    size: (usize, usize),
    buffer: Buf,
    cursor_controller: CursorController,
    editor_rows: EditorRows,
}

impl Output {
    pub fn new() -> Self {
        let size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            size,
            buffer: Buf::new(),
            cursor_controller: CursorController::new(size),
            editor_rows: EditorRows::new(),
        }
    }

    pub fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn draw_rows(&mut self) {
        let display_x = self.size.0;
        let display_y = self.size.1;
        let buffer_x = self.cursor_controller.col_offset;
        let buffer_y = self.cursor_controller.row_offset;
        let buffer_length = self.editor_rows.num_rows();
        let line_marker = "🔥";
        for i in 0..display_y {
            let rend_y = i + buffer_y;
            if i >= buffer_length {
                self.buffer.push_str(line_marker);
                if i == display_y / 10 && buffer_length == 0 {
                    let mut welcome = format!("🔥 Editor --- Version {}", VERSION);
                    if welcome.len() > display_x as usize {
                        welcome.truncate(display_x as usize);
                    }
                    let padding = (display_x as usize - welcome.len()) / 2;
                    (line_marker.len()..padding).for_each(|_| self.buffer.push_chr(' '));
                    self.buffer.push_str(&welcome);
                }
            } else {
                let (row_len, render_row) = if rend_y >= buffer_length {
                    (0, 0)
                } else {
                    (self.editor_rows.get_row(rend_y).len(), rend_y)   
                };
                let len = min(row_len.saturating_sub(buffer_x), display_x);
                let start = if len == 0 { 0 } else { buffer_x };
                self.buffer.push_str(&self.editor_rows.get_row(render_row)[start..start + len])
            }
            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < display_y - 1 {
                self.buffer.push_str("\n\r");
            }
        }
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.clear_screen()?;
        self.cursor_controller.scroll();
        queue!(self.buffer, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        
        // cursor_{x,y} is the position in the actual text buffer
        //  adjust be offsetting
        let cursor_x = self.cursor_controller.cursor_x - self.cursor_controller.col_offset;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;

        queue!(
            self.buffer,
            cursor::MoveTo(
                cursor_x as u16,
                cursor_y as u16
            )
        )?;
        self.buffer.flush()
    }

    pub fn move_cursor(&mut self, direction: event::KeyCode) {
        self.cursor_controller.move_cursor(direction, &self.editor_rows)
    }
}
