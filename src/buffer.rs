use std::io::stdout;

pub struct Buf {
    content: String,
}

impl Buf {
    pub fn new() -> Self {
        Self { content: String::new(), }
    }
    
    pub fn push_chr(&mut self, chr: char) {
        self.content.push(chr)
    }

    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str)
    }
}

impl std::io::Write for Buf {
    fn write(&mut self, ext_buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(ext_buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            },
            Err(_) => Err(std::io::ErrorKind::InvalidData.into())
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

