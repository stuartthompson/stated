pub struct CoreData<'a> {
    cols: u16,
    rows: u16,
    file_path: &'a str,
}

impl<'a> CoreData<'a> {
    pub fn new() -> CoreData<'a> {
        CoreData {
            cols: 0,
            rows: 0,
            file_path: "",
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
