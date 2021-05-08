mod bars;
mod core_data;
mod screens;
mod program;

use core_data::{CoreData};
use program::Program;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType, size},
    Command, Result,
};

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = std::io::stdout();

    let mut program: Program = Program::new();
    program.run(&mut stderr)
}
