use std::io::{self, Write};
mod bars;
mod screens;

use bars::{EditorInfoBar, StatusBar};
use screens::home_screen;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType, size},
    Command, Result,
};

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let (cols, rows) = size()?;

    // Create bars
    let editor_info_bar: EditorInfoBar = EditorInfoBar::new(rows - 1, cols, rows);
    let status_bar: StatusBar = StatusBar::new(rows - 2, "test.md");

    // Render bars
    editor_info_bar.render(w);
    status_bar.render(w);

    // Render current screen
    home_screen::render(w);

    loop {
        match read_char()? {
            '1' => {
                queue!(
                    w,
                    cursor::MoveTo(1, 1),
                    style::SetForegroundColor(style::Color::White),
                    style::SetBackgroundColor(style::Color::Black),
                    style::Print("Edit"),
                )?
            },
            'q' => break,
            _ => {}
        };

        w.flush()?;
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}
