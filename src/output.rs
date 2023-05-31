use crate::buffer::Buf;
use crate::cursor_controller::CursorController;
use crate::global_vars::VERSION;
use crate::rows::EditorRows;

use std::cmp::{min, max};
use std::io::{stdout, Write};

use crossterm::{cursor, event::{self, KeyCode}, execute, queue, terminal};
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
        let line_marker = "💩";
        for i in 0..display_y {
            let file_row_pos = i + self.cursor_controller.row_offset;
            let file_col_pos = self.cursor_controller.col_offset;
            if file_row_pos >= self.editor_rows.num_rows() {
                self.buffer.push_str(line_marker);
                if i == display_y / 10 && self.editor_rows.num_rows() == 0 {
                    let mut welcome = format!("Poop Editor --- Version {}", VERSION);
                    if welcome.len() > display_x as usize {
                        welcome.truncate(display_x as usize);
                    }
                    let padding = (display_x as usize - welcome.len()) / 2;
                    (line_marker.len()..padding).for_each(|_| self.buffer.push_chr(' '));
                    self.buffer.push_str(&welcome);
                }
            } else {
                let len = min(self.editor_rows.get_row(file_row_pos).len().saturating_sub(file_col_pos), display_x);
                if len == 0 {
                    self.buffer.push_str("")
                } else {
                    self.buffer
                        .push_str(&self.editor_rows.get_row(file_row_pos)[file_col_pos..(file_col_pos + len)])
                }
            }
            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < display_y - 1 {
                self.buffer.push_str("\n\r");
            }
        }
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.clear_screen()?;
        queue!(self.buffer, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        queue!(
            self.buffer,
            cursor::MoveTo(
                self.cursor_controller.cursor_x as u16,
                self.cursor_controller.cursor_y as u16
            )
        )?;
        self.buffer.flush()
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        let v_lim = self.editor_rows.num_rows();
        let at =
        match direction {
            KeyCode::Char('l') | KeyCode::Char('h') | KeyCode::Left | KeyCode::Right
                => self.cursor_controller.row_offset,
            KeyCode::Char('j') | KeyCode::Down
                => min(v_lim, self.cursor_controller.row_offset + 1),
            _   => self.cursor_controller.row_offset.saturating_sub(1),
        };

        self.cursor_controller
            .move_cursor(direction, self.editor_rows.num_rows(), self.editor_rows.get_row(at).len())
    }
}
