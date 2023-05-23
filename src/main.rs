use std::io;
use io::Read;

use crossterm::terminal;

struct Editor;

impl Editor {
    fn new(&self) {
        terminal::enable_raw_mode().expect("Could not disable raw mode");
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}



fn main() {
    let _e = Editor;
    _e.new();
    let mut stdin = io::stdin();
    let ref_stdin = io::BufReader::new(&mut stdin);

    for byte in ref_stdin.bytes() {
        let char = byte.unwrap() as char;
        if char.is_control() {
            println!("{}\r", char as u8);

        } else {
            println!("{}\r", char);
        }
        if char == 'q' {
            break;
        }
    }

    for byte in (&mut stdin).bytes() {
        println!("{}\r", byte.as_ref().unwrap());
        if byte.unwrap() == b'w' {
            break;
        }
    }

    for byte in stdin.bytes() {
        println!("{}\r", byte.as_ref().unwrap());
        if byte.unwrap() == b'e' {
            break;
        }
    }
    println!("Hello, world!\n\r");
}
