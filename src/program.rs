use std::io::Write;
use std::time::Duration;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, queue, style, terminal,
    Result
};

use crate::CoreData;
use crate::bars::{Bar, PerformanceBar, StatusBar};
use crate::editor::{Editor, Dimensions};
use crate::screens::home_screen;

pub struct Program<'a> {
    core_data: CoreData,
    // TODO: This should be a vector of editors.
    editor: Editor<'a>,
    bars: Vec<Box<dyn Bar>>,
    running: bool,
}

impl<'a> Program<'a> {
    /// Program initialization
    pub fn new() -> Program<'a> {
        Program {
            core_data: CoreData::new(),
            // TODO: Should be a vector of editors
            editor: Editor::new(Dimensions::default()),
            bars: Vec::new(),
            running: false,
        }
    }

    /// Runs the program
    pub fn run<W>(&mut self, w: &mut W) -> Result<()>
    where
        W: Write
    {
        execute!(w, terminal::EnterAlternateScreen)?;

        terminal::enable_raw_mode()?;

        self.running = true;

        // Create bars
        self.create_bars();

        // Render current screen
        home_screen::render(w);

        while self.running {
            // Check for user input
            self.check_input();

            // Render bars
            self.render_bars(w);

            queue!(
                w,
                cursor::MoveTo(self.editor.cursor_location.column_ix, self.editor.cursor_location.row_ix)
            )?;

            // Flush render queue
            w.flush()?;

            // Tick
            self.core_data.tick();

            // Update cursor location
            self.core_data.update_cursor_location(&self.editor.cursor_location);
        }
        execute!(
            w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        terminal::disable_raw_mode()
    }

    /// Checks for user input
    fn check_input(&mut self) {
        if poll(Duration::from_millis(17)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    if event == KeyCode::Char('h').into() {
                        // TODO: Should move cursor within active editor
                        self.editor.move_cursor_left(1);
                    }
                    if event == KeyCode::Char('j').into() {
                        // TODO: Should move cursor within active editor
                        self.editor.move_cursor_down(1);
                    }
                    if event == KeyCode::Char('k').into() {
                        // TODO: Should move cursor within active editor
                        self.editor.move_cursor_up(1);
                    }
                    if event == KeyCode::Char('l').into() {
                        // TODO: Should move cursor within active editor
                        self.editor.move_cursor_right(1);
                    }
                    if event == KeyCode::Char('q').into() {
                        self.running = false;
                    }
                },
                Event::Mouse(_) => {},
                Event::Resize(width, height) =>
                    self.handle_resize(width, height)
            }
        }
    }

    /// Handles window resize events
    fn handle_resize(&mut self, width: u16, height: u16) {
        // TODO: Should resize all editors according to their allocations
        self.editor.resize(Dimensions::new(width, height));
        self.core_data.update_dimensions(&self.editor.dimensions);
    }

    /// Creates status bars
    fn create_bars(&mut self) {
        self.bars.push(Box::new(StatusBar::new(1)));
        self.bars.push(Box::new(PerformanceBar::new(2)));
    }

    /// Renders status bars
    fn render_bars<W>(&mut self, w: &mut W)
    where
        W: Write
    {
        // TODO: Render bars at bottom, according to their priority
        let mut bar_row = 22; // TODO: <-- This should not be hard-coded
        for bar in &self.bars {
            queue!(
                w,
                cursor::MoveTo(0, bar_row),
                terminal::Clear(terminal::ClearType::CurrentLine),
                style::SetForegroundColor(style::Color::White),
                style::SetBackgroundColor(style::Color::Black),
                style::Print(bar.render(&self.core_data)),
            ).unwrap();
            bar_row += 1;
        }
    }
}
