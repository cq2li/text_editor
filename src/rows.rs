use std::{env, fs::{self, read_to_string}, io, cmp, path};

pub const TAB_STOP: usize = 8;

pub struct Row {
    pub row_content: Box<str>,
    pub render: String,
}

impl Row {
    fn new(row_content: Box<str>, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }
}

pub struct EditorRows {
    pub contents: Vec<Row>,
}

impl EditorRows {
    pub fn new() -> Self {
        let mut arg = env::args();
        match arg.nth(1) {
            None => Self {
                contents: Vec::new(),
            },
            Some(file) => Self::from_file(file.as_ref()),
        }
    }
    
    fn render_row(row: &mut Row) {
        let mut idx = 0;
        let capacity = 
            row.row_content
                .chars()
                .fold(0, |acc, next| acc + if next == '\t' { 8 } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content
            .chars()
            .for_each(|chr| {
                idx += 1;
                if chr == '\t' {
                    row.render.push(' ');
                    while idx % TAB_STOP != 0 {
                        row.render.push(' ');
                        idx += 1
                    }
                } else {
                    row.render.push(chr);
                }
            });
    }

    fn from_file(file: &path::Path) -> Self {
        let file_contents = read_to_string(file).expect("Unable to read");
        Self {
            contents: file_contents
                .lines()
                .map(|it| {
                    let mut row = Row::new(it.into(), String::new());
                    Self::render_row(&mut row);
                    row
                })
            .collect(),
        }
    }

    pub fn num_rows(&self) -> usize {
        self.contents.len()
    }

    pub fn get_row<'a>(self:&'a Self, at: usize) -> &'a Row {
        &self.contents[at]
    }

    pub fn get_render<'a>(self:&'a Self, at: usize) -> &'a str {
        self.get_row(at).render.as_str()
    }
}


