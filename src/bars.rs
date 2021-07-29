use crate::CoreData;

mod performance_bar;
mod status_bar;

pub use performance_bar::PerformanceBar;
pub use status_bar::StatusBar;

/// Bars render status information
pub trait Bar {
    /// Renders the bar
    fn render(&self, core_data: &CoreData) -> String;
}
