use crate::CoreData;
use super::Bar;

/// Displays performance statistics.
pub struct PerformanceBar {
    /// Bar priority. Used to determine the order in which bars are rendered
    ///   within the application. The smaller the priority number, the further
    ///   towards the top of the application the bar is rendered.
    priority: u8,
}

impl Bar for PerformanceBar {
    /// Renders the performance bar
    fn render(&self, core_data: &CoreData) -> String {
        format!("[Performance] Uptime (secs): {} Frames: {} FPS: {}",
        core_data.uptime(), core_data.frames(), core_data.fps())
    }
}

impl PerformanceBar {
    pub fn new(priority: u8) -> PerformanceBar
    {
        PerformanceBar {
            priority,
        }
    }
}
