use std::{env, fs::{self, read_to_string}, io, cmp, path};

pub struct EditorRows {
    contents: Vec<Box<str>>,
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

    fn from_file(file: &path::Path) -> Self {
        let file_contents = read_to_string(file).expect("Unable to read");
        Self {
            contents: file_contents.lines().map(|it| it.into()).collect(),
        }
    }

    pub fn num_rows(&self) -> usize {
        self.contents.len()
    }

    pub fn get_row<'a>(self:&'a Self, at: usize) -> &'a str {
        &self.contents[at]
    }
}


