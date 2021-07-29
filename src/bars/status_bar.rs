use crate::CoreData;
use super::Bar;

pub struct StatusBar {
    /// Bar priority. Used to determine the order in which bars are rendered
    ///   within the application. The smaller the priority number, the further
    ///   towards the top of the application the bar is rendered.
    priority: u8,
}

impl Bar for StatusBar {
    fn render(&self, core_data: &CoreData) -> String {
        format!("[Status] File path: FILE_PATH_HERE")
    }
}

impl StatusBar {
    pub fn new(priority: u8) -> StatusBar {
        StatusBar {
            priority,
        }
    }
}
