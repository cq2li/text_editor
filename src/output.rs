use crate::buffer::Buf;
use crate::cursor_controller::CursorController;
use crate::global_vars::VERSION;
use crate::rows::EditorRows;
use crate::status::StatusMessage;

use std::cmp::{max, min};
use std::io::{self, stdout, Write};

use crossterm::style;
use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, queue, terminal,
};
use terminal::ClearType;

pub struct Output {
    size: (usize, usize),
    buffer: Buf,
    cursor_controller: CursorController,
    editor_rows: EditorRows,
    status_message: StatusMessage,
}

impl Output {
    pub fn new() -> Self {
        let size = terminal::size()
            .map(|(x, y)| (x as usize, (y - 1) as usize))
            .unwrap();
        Self {
            size,
            buffer: Buf::new(),
            cursor_controller: CursorController::new(size),
            editor_rows: EditorRows::new(),
            status_message: StatusMessage::new("HELP: CTRL + q to exit"),
        }
    }

    pub fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn draw_rows(&mut self) {
        // the terminal size [)
        let display_x = self.size.0;
        let display_y = self.size.1;
        // the position in the buffer of the cursor [)
        let buffer_x = self.cursor_controller.col_offset;
        let buffer_y = self.cursor_controller.row_offset;
        let buffer_length = self.editor_rows.num_rows();
        let line_marker = "🔥";
        for i in 0..display_y {
            let rend_y = i + buffer_y;
            if i >= buffer_length {
                self.buffer.push_str(line_marker);
                if i == display_y / 20 && buffer_length == 0 {
                    let mut welcome = format!("🔥 Editor --- Version {}", VERSION);
                    if welcome.len() > display_x as usize {
                        welcome.truncate(display_x as usize);
                    }
                    let padding = (display_x as usize - welcome.len()) / 2;
                    (2..padding).for_each(|_| self.buffer.push_chr(' '));
                    self.buffer.push_str(&welcome);
                }
                if i == display_y / 20 + 2 && buffer_length == 0 {
                    let mut welcome = format!("CTRL + q to quit");
                    if welcome.len() > display_x as usize {
                        welcome.truncate(display_x as usize);
                    }
                    let padding = (display_x as usize - welcome.len()) / 2;
                    (0..padding).for_each(|_| self.buffer.push_chr(' '));
                    self.buffer.push_str(&welcome);
                }
            } else {
                // determine row_length and the row to be rendered
                let (row_len, render_row) = if rend_y >= buffer_length {
                    (0, 0)
                } else {
                    (self.editor_rows.get_render(rend_y).len(), rend_y)
                };
                let len = min(row_len.saturating_sub(buffer_x), display_x);
                let start = if len == 0 { 0 } else { buffer_x };
                self.buffer
                    .push_str(&self.editor_rows.get_render(render_row)[start..start + len])
            }
            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < display_y {
                self.buffer.push_str("\n\r");
            }
        }
        // self.buffer.push_str("test");
    }

    pub fn draw_status_bar(&mut self) {
        self.buffer.push_str(&style::Attribute::Reverse.to_string());

        let info = match self.status_message.message() {
            Some(msg) => msg.into(),
            None => {
                format!(
                    "{} -- {} lines",
                    self.editor_rows
                        .filename
                        .as_ref()
                        .and_then(|path| path.file_name())
                        .and_then(|name| name.to_str())
                        .unwrap_or("[No Name]"),
                    self.editor_rows.num_rows()
                )
            }
        };
        let info_len = min(info.len(), self.size.0);
        let line_info: String = if self.editor_rows.num_rows() == 0 {
            String::from("Empty File")
        } else {
            let y_lim = self.editor_rows.num_rows();
            let buf_y = self.cursor_controller.cursor_y;
            let x_lim = 
                if y_lim == buf_y {
                    0
                } else {
                    self.editor_rows.get_row(buf_y).row_content.len()
                };
            let buf_x = self.cursor_controller.cursor_x;

            format!(
                "col {}/{} row {}/{}",
                buf_x + 1,
                x_lim + 1,
                buf_y + 1,
                y_lim + 1,
            )
        };
        let line_info_len = line_info.len();
        self.buffer.push_str(&info[..info_len]);
        (info_len..self.size.0 - line_info_len).for_each(|_| self.buffer.push_chr(' '));
        self.buffer.push_str(&line_info);
        self.buffer.push_str(&style::Attribute::Reset.to_string());
    }

    pub fn refresh(&mut self) -> crossterm::Result<()> {
        self.cursor_controller.scroll(&self.editor_rows);
        queue!(self.buffer, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        self.draw_status_bar();
        // cursor_{x,y} is the position in the actual text buffer
        //  adjust be offsetting
        let cursor_x = self.cursor_controller.render_x - self.cursor_controller.col_offset;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;

        queue!(
            self.buffer,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16)
        )?;
        self.buffer.flush()
    }

    pub fn move_cursor(&mut self, direction: event::KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &self.editor_rows)
    }

    pub fn insert_char(&mut self, char: char) {
        if self.cursor_controller.cursor_y == self.editor_rows.num_rows() {
            self.editor_rows.insert_row();
        }
        self.editor_rows
            .get_row_mut(self.cursor_controller.cursor_y)
            .insert_char(char, self.cursor_controller.cursor_x);
        self.cursor_controller.cursor_x += 1;
    }

    pub fn backspace(&mut self) {
        let cursor_y = self.cursor_controller.cursor_y;
        let cursor_x = self.cursor_controller.cursor_x;
        let total_rows = self.editor_rows.num_rows();

        let content_len = if cursor_y < total_rows {
            self.editor_rows.get_row(cursor_y).row_content.len()
        } else {
            0
        };

        match (cursor_y, cursor_x, total_rows) {
            // top left cursor with empty file, delete empty row
            (0, 0, 1) if content_len == 0 => 
                self.editor_rows.delete_row(cursor_y),
            // top left cursor with non empty file, do nothing
            (0, 0, _) => (),
            // if at the end of the file, nothing to delete so move cursor left
            (c_y, _, t_r) if c_y == t_r => 
                self.move_cursor(KeyCode::Left),
            // if at the beginning of a line, "delete" the new line by 
            //  appending row to previous row and then deleting the current row
            (c_y, 0, _) => {
                let prev_row_len = 
                    self.editor_rows.get_row(cursor_y - 1).row_content.len();

                self.editor_rows.delete_row_shift_up(c_y);
                self.cursor_controller.cursor_y -= 1;
                self.cursor_controller.cursor_x = prev_row_len;
                ()
            },
            // normal deletion of a character before the x cursor
            (c_y, c_x, _) => {
                self.editor_rows.get_row_mut(c_y).delete_char(c_x - 1);
                self.cursor_controller.cursor_x -= 1;
                ()
            }
        }
    }
    
    /* @brief peforms text deletion when the delete key is pressed
     */
    pub fn del(&mut self) {
        let cursor_y = self.cursor_controller.cursor_y;
        let cursor_x = self.cursor_controller.cursor_x;
        let total_rows = self.editor_rows.num_rows();
        let content_len = if cursor_y < total_rows {
            self.editor_rows.get_row(cursor_y).row_content.len()
        } else {
            0
        };

        match (cursor_y, cursor_x, total_rows) {
            // empty file, do nothing
            (0, 0, 0) => (),
            // empty row, delete it and make it an empty file
            (0, 0, 1) if content_len == 0 => self.editor_rows.delete_row(0),
            // end of the file, do nothing
            (c_y, c_x, t_r) if c_y >= t_r - 1 && c_x == content_len => (),
            // pressing delete at the end of an row concats two lines
            (c_y, c_x, _) if c_x == content_len => {
                self.editor_rows.delete_row_shift_up(c_y + 1);
            },
            // normal deletion of a character before the x cursor
            (c_y, c_x, _) => {
                self.editor_rows.get_row_mut(c_y).delete_char(c_x);
                ()
            },

        }

    } 
    
    pub fn enter(&mut self) {
        let cursor_y = self.cursor_controller.cursor_y;
        let cursor_x = self.cursor_controller.cursor_x;
        let total_rows = self.editor_rows.num_rows();
        let content_len = if cursor_y < total_rows {
            self.editor_rows.get_row(cursor_y).row_content.len()
        } else {
            0
        };

        if cursor_y == total_rows {
            self.editor_rows.insert_row();
            self.cursor_controller.cursor_y += 1;
        } else {
            let split = 
                self.editor_rows.get_row_mut(cursor_y).row_content.split_off(cursor_x);
            self.editor_rows.insert_row_at(cursor_y + 1);
            self.editor_rows.get_row_mut(cursor_y + 1).row_content.push_str(&split);
            EditorRows::render_row(self.editor_rows.get_row_mut(cursor_y));
            EditorRows::render_row(self.editor_rows.get_row_mut(cursor_y + 1));
        }
    }

    pub fn save(&self) {
        self.editor_rows.save().unwrap()
    }
}
