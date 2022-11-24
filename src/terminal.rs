use std::io::{self, Write, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    // Here we are using termion  to provide stdout.
    // We are assigning the result of "into_raw_mode" to a variable named _stdout
    // into_raw_mode modifies the terminal and returns a value which, once it is removed,
    // will reset the terminal into canonical mode - so we are keeping this variable around
    // by binding it to _stdout.
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {

    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(
            Self { 
                size: Size {
                    width: size.0,
                    height: size.1,
                },  
                _stdout: stdout().into_raw_mode()?,
            }
        )
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}