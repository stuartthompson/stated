use std::io::Write;
use crossterm::{cursor, queue, style};

/// Renders the home screen.
pub fn render<W>(w: &mut W)
where
    W: Write,
{
    queue!(
        w,
        cursor::MoveTo(0, 0),
        style::Print("Stu Thompson's Awesome Text Editor")
    ).unwrap();

    w.flush().unwrap();
}
