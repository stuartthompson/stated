use std::io::Write;
use crossterm::{cursor, queue, style};

pub struct StatusBar<'a> {
    render_row_ix: u16,
    file_path: &'a str,
}

impl<'a> StatusBar<'a> {
    pub fn new(render_row_ix: u16, file_path: &'a str) -> StatusBar {
        StatusBar {
            render_row_ix,
            file_path
        }
    }

    /// Renders the status bar
    pub fn render<W>(&self, w: &mut W)
    where
        W: Write,
    {
        queue!(
            w,
            cursor::MoveTo(0, self.render_row_ix),
            style::Print("Status Bar"),
            style::Print(format!("File path: {}", self.file_path)),
        ).unwrap();
    }
}
