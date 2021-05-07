use std::io::Write;
use crossterm::{cursor, queue, style};
use crate::CoreData;

pub struct StatusBar<'a> {
    render_row_ix: u16,
    file_path: &'a str,
}

impl<'a> StatusBar<'a> {
    pub fn new(render_row_ix: u16) -> StatusBar<'a> {
        StatusBar {
            render_row_ix,
            file_path: ""
        }
    }

    /// Renders the status bar
    pub fn render<W>(&self, w: &mut W, core_data: &CoreData)
    where
        W: Write,
    {
        queue!(
            w,
            cursor::MoveTo(0, self.render_row_ix),
            style::Print("Status Bar"),
            style::Print(format!("File path: {}", core_data.get_file_path())),
        ).unwrap();
    }
}
