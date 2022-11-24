#![warn(clippy::all, clippy::pedantic)]
use editor::Editor;

mod editor;
mod terminal;
pub use terminal::Terminal;
/*
    Following Philipp Flenker tutorial to build a text editor in Rust, to learn more about it
    Found here: https://www.philippflenker.com/hecto/
*/
fn main() {
    Editor::default().run();
}