use std::time::{Duration, SystemTime};

pub struct CoreData<'a> {
    cols: u16,
    rows: u16,
    file_path: &'a str,
    start_time: SystemTime,
    running_for_secs: u64,
    frames: u64
}

impl<'a> CoreData<'a> {
    pub fn new() -> CoreData<'a> {
        CoreData {
            start_time: SystemTime::now(),
            running_for_secs: 0,
            frames: 0,
            cols: 0,
            rows: 0,
            file_path: "",
        }
    }

    pub fn tick(&mut self) {
        let now = SystemTime::now();
        let time: Duration = now.duration_since(self.start_time).unwrap();
        self.running_for_secs = time.as_secs();
        self.frames += 1;
    }

    pub fn get_frames(&self) -> u64 {
        self.frames
    }

    pub fn get_uptime(&self) -> u64 {
        self.running_for_secs
    }

    pub fn get_fps(&self) -> u64 {
        if self.running_for_secs == 0 {
            return 0;
        } else {
            return self.frames / self.running_for_secs;
        }
    }

    pub fn set_dimensions(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    pub fn set_file_path(&mut self, file_path: &'a str) {
        self.file_path = file_path;
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        return (self.cols, self.rows);
    }

    pub fn get_file_path(&self) -> &str {
        self.file_path
    }
}
