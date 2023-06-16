use crate::buffer::Buf;
use crate::cursor_controller::CursorController;
use crate::global_vars::VERSION;
use crate::rows::EditorRows;
use crate::status::StatusMessage;
use std::cmp::min;
use std::io::{stdout, Write};
use crossterm::style;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyModifiers},
    execute, queue, terminal,
};
use terminal::ClearType;
use crate::reader::Reader;
use crate::search::{ SearchIndex, SearchDirection };

pub struct Output {
    size: (usize, usize),
    buffer: Buf,
    cursor_controller: CursorController,
    pub editor_rows: EditorRows,
    pub status_message: StatusMessage,
    pub dirty: u64,
    search_index: SearchIndex,
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
            status_message: StatusMessage::new("HELP: CTRL + {q: exits, s: save, f: search}"),
            dirty: 0,
            search_index: SearchIndex::new(),
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
                    "{} {} -- {} lines",
                    self.editor_rows
                        .filename
                        .as_ref()
                        .and_then(|path| path.file_name())
                        .and_then(|name| name.to_str())
                        .unwrap_or("[No Name]"),
                    if self.dirty > 0 { "Modified!" } else { "" },
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
            let x_lim = if y_lim == buf_y {
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
        self.dirty += 1;
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
            (0, 0, 1) if content_len == 0 => self.editor_rows.delete_row(cursor_y),
            // top left cursor with non empty file, do nothing
            (0, 0, _) => (),
            // if at the end of the file, nothing to delete so move cursor left
            (c_y, _, t_r) if c_y == t_r => self.move_cursor(KeyCode::Left),
            // if at the beginning of a line, "delete" the new line by
            //  appending row to previous row and then deleting the current row
            (c_y, 0, _) => {
                let prev_row_len = self.editor_rows.get_row(cursor_y - 1).row_content.len();

                self.editor_rows.delete_row_shift_up(c_y);
                self.cursor_controller.cursor_y -= 1;
                self.cursor_controller.cursor_x = prev_row_len;
                ()
            }
            // normal deletion of a character before the x cursor
            (c_y, c_x, _) => {
                self.editor_rows.get_row_mut(c_y).delete_char(c_x - 1);
                self.cursor_controller.cursor_x -= 1;
                ()
            }
        }
        self.dirty += 1;
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
            }
            // normal deletion of a character before the x cursor
            (c_y, c_x, _) => {
                self.editor_rows.get_row_mut(c_y).delete_char(c_x);
                ()
            }
        }
        self.dirty += 1;
    }

    pub fn enter(&mut self) {
        let cursor_y = self.cursor_controller.cursor_y;
        let cursor_x = self.cursor_controller.cursor_x;
        let total_rows = self.editor_rows.num_rows();

        if cursor_y == total_rows {
            self.editor_rows.insert_row();
        } else {
            let split = self
                .editor_rows
                .get_row_mut(cursor_y)
                .row_content
                .split_off(cursor_x);
            self.editor_rows.insert_row_at(cursor_y + 1);
            self.editor_rows
                .get_row_mut(cursor_y + 1)
                .row_content
                .push_str(&split);
            EditorRows::render_row(self.editor_rows.get_row_mut(cursor_y));
            EditorRows::render_row(self.editor_rows.get_row_mut(cursor_y + 1));
        }
        self.cursor_controller.cursor_y += 1;
        self.cursor_controller.cursor_x = 0;
        self.dirty += 1;
    }

    pub fn save(&mut self) {
        let res = self.editor_rows.save();
        match res {
            Err(err) => {
                self.status_message.clear_custom_message();
                self.status_message.set_message(format!("Error {:?}", err.to_string()));
            },
            Ok(w) => {
                self.dirty = 0;
                self.status_message
                    .set_message(format!("{} bytes written to disk", w))

            }
        }
    }
    
    fn find_callback(output: &mut Output, keyword: &str, key_code: Option<KeyCode>) {
        match key_code {
            Some(KeyCode::Esc | KeyCode::Enter) => {
                output.search_index.reset();
            },
            Some(_) => {
                // these two lines reset the position of search when a new character is typed
                output.search_index.y_direction = None;
                output.search_index.x_direction = None;
                match key_code {
                    Some(KeyCode::Down) => {
                        output.search_index.y_direction = SearchDirection::Forward.into()
                    },
                    Some(KeyCode::Up) => {
                        output.search_index.y_direction = SearchDirection::Backward.into()
                    },
                    Some(KeyCode::Left) => {
                        output.search_index.x_direction = SearchDirection::Backward.into()
                    },
                    Some(KeyCode::Right) => {
                        output.search_index.x_direction = SearchDirection::Forward.into()
                    },
                    _ => {}
                } 
                // resets anytime any key other than up/down/left/right is pressed
                for i in 0..output.editor_rows.num_rows() {
                    let row_index = match output.search_index.y_direction {
                        None => {
                            if output.search_index.x_direction.is_none() {
                                output.search_index.y_index = i;
                            }
                            output.search_index.y_index
                        },
                        Some(ref dir) => {
                            if matches!(dir, SearchDirection::Forward) {
                                output.search_index.y_index + i + 1
                            } else {
                                let res = output.search_index.y_index.saturating_sub(i);
                                if res == 0 {
                                    break;
                                }
                                res - 1
                            }
                        }
                    };
                    if row_index >= output.editor_rows.num_rows() { break; }
                    let row = output.editor_rows.get_row(row_index);

                    // resets when up/down is pressed
                    let index = match output.search_index.x_direction {
                        None => row.render.find(&keyword),
                        Some(ref dir) => {
                            let index = if matches!(dir, SearchDirection::Forward) {
                                let start = min(row.render.len(), output.search_index.x_index + 1);
                                row.render[start..].find(&keyword).and_then(|index| Some(start + index))
                            } else {
                                row.render[..output.search_index.x_index].rfind(&keyword)
                            };
                            if index.is_none() {
                                break;
                            }
                            index
                        }
                    };

                    if let Some(index) = index {
                        output.cursor_controller.cursor_y = row_index;
                        output.search_index.y_index = row_index;
                        output.search_index.x_index = index;
                        output.cursor_controller.cursor_x = output.cursor_controller.get_render_x(row, index);
                        break;
                    }
                }
            }
            None => (),
        }
    }
    

    pub fn find(&mut self) -> std::io::Result<()> {
        let res = self.cursor_controller.clone();
        if prompt!(self, "Search: {} (ESC to Cancel, Enter to Confirm)", Output::find_callback).is_none() {
            self.cursor_controller = res;
        };
        Ok(())
    }
}

#[macro_export()]
macro_rules! prompt {
    ($output:expr, $args:tt) => {
        prompt!($output, $args, |&_, _, _| {}) // this ignores the callback when called
                                                          // with 2 arguments
    };
    ($output:expr, $args:tt, $callback:expr) => {{
        let output:&mut Output = $output;
        // file name length
        let mut input: String = String::with_capacity(255);
        loop {
            output.status_message.set_message(format!($args, input));
            output.refresh()?;
            let key_event = Reader.read_keyevent();
            match key_event {
                Some(event::KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if !input.is_empty() {
                        output.status_message.set_message(String::new());
                        $callback(output, &input, Some(KeyCode::Enter));
                        break;
                    }
                },
                Some(event::KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    if input.len() > 0 {
                        input.pop();
                    }
                },
                Some(event::KeyEvent {
                    code: code @ KeyCode::Char(..),
                    modifiers: case @ (KeyModifiers::NONE| KeyModifiers::SHIFT),
                    ..
                }) => input.push(
                    match code {
                        KeyCode::Char(char) if matches!(case, KeyModifiers::NONE) => char,
                        KeyCode::Char(char) if matches!(case, KeyModifiers::SHIFT) => char.to_ascii_uppercase(),
                        _ => unimplemented!(),
                    }
                ),
                Some(event::KeyEvent {
                    code: KeyCode::Esc,
                    ..
                }) => {
                    input.clear();
                    output.status_message.clear_custom_message();
                    $callback(output, &input, Some(KeyCode::Esc));
                    break;
                },
                _ => (),
            }
            $callback(output, &input, key_event.and_then(|event| Some(event.code)));
        }
        if input.is_empty() { None } else { Some(input) }
    }};

}

use prompt;
