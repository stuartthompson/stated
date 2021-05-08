use crate::CoreData;

mod editor_info_bar;
mod status_bar;

pub use editor_info_bar::EditorInfoBar;
pub use status_bar::StatusBar;

/// Bars render status information
pub trait Bar {
    /// Renders the bar
    fn render(&self, core_data: &CoreData) -> String;
}
