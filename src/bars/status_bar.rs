use crate::CoreData;
use super::Bar;

pub struct StatusBar {
    render_row_ix: u16,
}

impl Bar for StatusBar {
    fn render(&self, core_data: &CoreData) -> String {
        format!("[Status] File path: {}", core_data.get_file_path())
    }
}

impl StatusBar {
    pub fn new(render_row_ix: u16) -> StatusBar {
        StatusBar {
            render_row_ix,
        }
    }
}
