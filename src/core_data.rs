use std::time::{Duration, SystemTime};
use crate::editor::{Dimensions,Location};

pub struct CoreData {
    start_time: SystemTime,
    running_for_secs: u64,
    frames: u64,
    dimensions: Dimensions,
    cursor_location: Location,
}

impl CoreData {
    pub fn new() -> CoreData {
        CoreData {
            start_time: SystemTime::now(),
            running_for_secs: 0,
            frames: 0,
            dimensions: Dimensions::default(),
            cursor_location: Location::default()
        }
    }

    pub fn tick(&mut self) {
        let now = SystemTime::now();
        let time: Duration = now.duration_since(self.start_time).unwrap();
        self.running_for_secs = time.as_secs();
        self.frames += 1;
    }

    pub fn frames(&self) -> u64 {
        self.frames
    }

    pub fn uptime(&self) -> u64 {
        self.running_for_secs
    }

    pub fn fps(&self) -> u64 {
        if self.running_for_secs == 0 {
            return 0;
        } else {
            return self.frames / self.running_for_secs;
        }
    }

    pub fn dimensions(&self) -> &Dimensions {
        return &self.dimensions;
    }

    pub fn update_dimensions(&mut self, dimensions: &Dimensions) {
        self.dimensions = Dimensions::from(dimensions);
    }

    pub fn cursor_location(&self) -> &Location {
        return &self.cursor_location;
    }

    pub fn update_cursor_location(&mut self, location: &Location) {
        self.cursor_location = Location::from(location);
    }
}