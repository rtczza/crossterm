//! This is an ANSI specific implementation for the screen manager
//! This module is used for windows 10 terminals and unix terminals by default.
//! This module uses the stdout to write to the console.

use std::any::Any;
use std::io::{self, Write, Read };

use super::IScreenManager;

pub struct AnsiScreenManager {
    pub is_alternate_screen: bool,
    output: Box<Write>,
    input: Box<Read>
}

impl IScreenManager for AnsiScreenManager {
    fn toggle_is_alternate_screen(&mut self, is_alternate_screen: bool) {
        self.is_alternate_screen = is_alternate_screen;
    }

    fn write_string(&mut self, string: String) -> io::Result<usize> {
        write!(self.output, "{}", string)?;
        self.flush();
        Ok(0)
    }

    fn write_str(&mut self, string: &str) -> io::Result<usize> {
        write!(self.output, "{}", string)?;
        self.flush();
        Ok(0)
    }

//    fn read_line(&mut self) -> io::Result<String>
//    {
//        let mut rv = String::new();
//        self.input.read_line(&mut rv)?;
//        let len = rv.trim_right_matches(&['\r', '\n'][..]).len();
//        rv.truncate(len);
//        Ok(rv)
//    }
//
//    fn read_char(&mut self) -> io::Result<String>
//    {
//
//    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }

    fn as_any(&mut self) -> &mut Any {
        self
    }
}

impl AnsiScreenManager {
    pub fn new() -> Self {
        AnsiScreenManager {
            input: (Box::from(io::stdin()) as Box<Read>),
            output: (Box::from(io::stdout()) as Box<Write>),
            is_alternate_screen: false,
        }
    }
}
