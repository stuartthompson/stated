use crate::CoreData;
use super::Bar;

pub struct EditorInfoBar {
    render_row_ix: u16,
}

impl Bar for EditorInfoBar {
    /// Renders the editor info bar
    fn render(&self, core_data: &CoreData) -> String {
        let (cols, rows) = core_data.get_dimensions();
        let uptime = core_data.get_uptime();
        let frames = core_data.get_frames();
        let fps = core_data.get_fps();
        format!("[Editor Info] Cols: {} Rows: {} Uptime (secs): {} Frames: {} FPS: {}",
            cols, rows, uptime, frames, fps)
    }
}

impl EditorInfoBar {
    pub fn new(render_row_ix: u16) -> EditorInfoBar
    {
        EditorInfoBar {
            render_row_ix,
        }
    }
}
