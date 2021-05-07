use std::io::Write;
use crossterm::{cursor, queue, style};

pub struct EditorInfoBar {
    render_row_ix: u16,
    cols: u16,
    rows: u16,
}

impl EditorInfoBar {
    pub fn new(render_row_ix: u16, cols: u16, rows: u16) -> EditorInfoBar
    {
        EditorInfoBar {
            render_row_ix: render_row_ix,
            cols: cols,
            rows: rows
        }
    }

    /// Renders the editor info bar
    pub fn render<W>(&self, w: &mut W)
    where
        W: Write
    {
        queue!(
            w,
            cursor::MoveTo(0, self.render_row_ix),
            style::Print("Editor Status"),
            style::Print(format!(" Cols: {} Rows: {}", self.cols, self.rows)),
        ).unwrap();
    }
}
