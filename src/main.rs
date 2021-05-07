use std::io::{self, Write};
mod bars;
mod core_data;
mod screens;

use bars::{EditorInfoBar, StatusBar};
use core_data::{CoreData, DimensionsSubscriber};
use screens::home_screen;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType, size},
    Command, Result,
};

pub struct Program<'a> {
    core_data: CoreData<'a>,
    editor_info_bar: EditorInfoBar,
    status_bar: StatusBar<'a>,
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        let core_data = CoreData::new();
        let editor_info_bar = EditorInfoBar::new(49);
        let status_bar = StatusBar::new(50);
        Program {
            core_data,
            editor_info_bar,
            status_bar
        }
    }

    pub fn render_bars<W>(&mut self, w: &mut W)
    where
        W: Write
    {
        self.editor_info_bar.render(w, &self.core_data);
        self.status_bar.render(w, &self.core_data);
    }
}

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

    // Create program
    let mut program: Program = Program::new();

    // Initialize core data
    let (cols, rows) = size()?;
    program.core_data.set_dimensions(cols, rows);
    program.core_data.set_file_path("test.md");

    // Render bars
    program.render_bars(w);

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
                )?;
                let (cols, rows) = size()?;
                program.core_data.set_dimensions(cols, rows);
            },
            'q' => break,
            _ => {}
        };

        // Render bars
        program.render_bars(w);

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
