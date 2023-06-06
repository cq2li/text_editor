use std::{env, fs::{self, read_to_string}, io, path::{PathBuf}, io::Write};

pub const TAB_STOP: usize = 8;

#[derive(Default)]
pub struct Row {
    pub row_content: String,
    pub render: String,
}

impl Row {
    fn new(row_content: String, render: String) -> Self {
        Self {
            row_content,
            render,
        }
    }

    pub fn insert_char(&mut self, char: char, at: usize) {
        self.row_content.insert(at, char);
        EditorRows::render_row(self);
    }

    pub fn delete_char(&mut self, at: usize) {
        if at < self.row_content.len() {
            self.row_content.remove(at);
            EditorRows::render_row(self)
        }
    }

}

pub struct EditorRows {
    contents: Vec<Row>,
    pub filename: Option<PathBuf>,
    
}

impl EditorRows {
    pub fn new() -> Self {
        let mut arg = env::args();
        match arg.nth(1) {
            None => Self {
                contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into()),
        }
    }
    
    pub fn render_row(row: &mut Row) {
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

    fn from_file(file: PathBuf) -> Self {
        let file_contents = read_to_string(&file).expect("Unable to read");
        Self {
            filename: Some(file),
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

    pub fn get_row_mut(&mut self, at: usize) -> &mut Row {
        &mut self.contents[at]
    }

    pub fn get_render<'a>(self:&'a Self, at: usize) -> &'a str {
        self.get_row(at).render.as_str()
    }

    pub fn insert_row(&mut self) {
        self.contents.push(Row::default())
    }

    pub fn delete_row(&mut self, at: usize) {
        self.contents.remove(at);
    }

    pub fn delete_row_shift_up(&mut self, at: usize) {
        let content = self.get_row(at).row_content.clone();
        let pushed_on = self.get_row_mut(at - 1);
        pushed_on
            .row_content
            .push_str(content.as_str());
        EditorRows::render_row(pushed_on);
        self.delete_row(at);
    }

    pub fn insert_row_at(&mut self, at: usize) {
        self.contents.insert(at, Row::default());
    }

    pub fn save(&self) -> io::Result<usize> {
        match &self.filename {
            None => Err(io::Error::new(io::ErrorKind::NotFound, "no file name specified")),
            Some(name) => {
                let mut file = fs::OpenOptions::new().write(true).create(true).open(name)?;
                let content = 
                    self.contents
                        .iter()
                        .fold(
                            String::new(),
                            |mut accm, row| {
                                accm.push_str(row.row_content.as_str());
                                accm.push('\n');
                                accm
                            });         
                file.set_len(content.len() as u64)?;
                if let Err(err) = file.write_all(content.as_bytes()) {
                    return Err(err);
                };
                Ok(content.as_bytes().len())
            }
        }
    }

}


