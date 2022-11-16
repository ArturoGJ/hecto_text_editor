use std::io::{stdout, self};

use termion::{event::Key, raw::IntoRawMode, input::TermRead};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        // Here we are using termion  to provide stdout.
        // We are assigning the result of "into_raw_mode" to a variable named _stdout
        // into_raw_mode modifies the terminal and returns a value which, once it is removed,
        // will reset the terminal into canonical mode - so we are keeping this variable around
        // by binding it to _stdout.
        let _stdout = stdout().into_raw_mode().unwrap();
        
        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self { should_quit: false }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?; // If there is an error return it, else keep going.
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{e}");
}