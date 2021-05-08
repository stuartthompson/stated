use std::io::Write;
use std::time::Duration;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, queue, style, terminal,
    Result
};

use crate::CoreData;
use crate::bars::{Bar, EditorInfoBar, StatusBar};
use crate::screens::home_screen;

pub struct Program<'a> {
    core_data: CoreData<'a>,
    bars: Vec<Box<dyn Bar>>,
    running: bool,
    cursor_x: u16,
    cursor_y: u16,
}

impl<'a> Program<'a> {
    /// Program initialization
    pub fn new() -> Program<'a> {
        Program {
            core_data: CoreData::new(),
            bars: Vec::new(),
            running: false,
            cursor_x: 0,
            cursor_y: 0,
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
                cursor::MoveTo(self.cursor_x, self.cursor_y)
            )?;

            // Flush render queue
            w.flush()?;

            // Tick
            self.core_data.tick();
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
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        }
                    }
                    if event == KeyCode::Char('j').into() {
                        if self.cursor_y < 48 {
                            self.cursor_y += 1;
                        }
                    }
                    if event == KeyCode::Char('k').into() {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    if event == KeyCode::Char('l').into() {
                        if self.cursor_x < 50 {
                            self.cursor_x += 1;
                        }
                    }
                    if event == KeyCode::Char('s').into() {
                        std::thread::sleep(Duration::from_millis(5000));
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
        self.core_data.set_dimensions(width, height);
    }

    /// Creates status bars
    fn create_bars(&mut self) {
        self.bars.push(Box::new(EditorInfoBar::new(49)));
        self.bars.push(Box::new(StatusBar::new(50)));
    }

    /// Renders status bars
    fn render_bars<W>(&mut self, w: &mut W)
    where
        W: Write
    {
        let mut bar_row = 48;
        for bar in &self.bars {
            queue!(
                w,
                cursor::MoveTo(1, bar_row),
                style::SetForegroundColor(style::Color::White),
                style::SetBackgroundColor(style::Color::Black),
                style::Print(bar.render(&self.core_data)),
            ).unwrap();
            bar_row += 1;
        }
    }
}
