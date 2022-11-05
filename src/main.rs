use std::io::{self, stdout};

use termion::{raw::IntoRawMode, event::Key, input::TermRead};

fn die(e: std::io::Error) {
    panic!("{e}");
}
/*
    Following Philipp Flenker tutorial to build a text editor in Rust, to learn more about it
    Found here: https://www.philippflenker.com/hecto/
*/
fn main() {
    // Here we are using termion  to provide stdout.
    // We are assigning the result of "into_raw_mode" to a variable named _stdout
    // into_raw_mode modifies the terminal and returns a value which, once it is removed,
    // will reset the terminal into canonical mode - so we are keeping this variable around
    // by binding it to _stdout.
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    // From wikipedia: In computing, a control character or non-printing character is 
                    // a code point (a number) in a character set, that does not represent a written 
                    // symbol. They are used as in-band signaling to cause effects other than the 
                    // addition of a symbol to the text.
                    if c.is_control() {
                        println!("{:?} \r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('c') => break, // This allows us to press Ctrl + c to exit.
                _ => println!("{:?}\r", key),
            },
            Err(err) => die(err),
        }
    }
}