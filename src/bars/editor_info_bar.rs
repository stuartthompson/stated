use std::io::Write;
use crossterm::{cursor, queue, style};
use crate::CoreData;

pub struct EditorInfoBar {
    render_row_ix: u16,
}

impl EditorInfoBar {
    pub fn new(render_row_ix: u16) -> EditorInfoBar
    {
        EditorInfoBar {
            render_row_ix,
        }
    }

    /// Renders the editor info bar
    pub fn render<W>(&self, w: &mut W, core_data: &CoreData)
    where
        W: Write
    {
        let (cols, rows) = core_data.get_dimensions();

        queue!(
            w,
            cursor::MoveTo(0, self.render_row_ix),
            style::Print("Editor Status"),
            style::Print(format!(" Cols: {} Rows: {}", cols, rows)),
        ).unwrap();
    }
}


