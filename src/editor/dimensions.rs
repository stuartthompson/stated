pub struct Dimensions {
    pub columns: u16,
    pub rows: u16,
}

impl Dimensions {
    pub fn new(columns: u16, rows: u16) -> Dimensions {
        Dimensions {
            columns: columns,
            rows: rows
        }
    }
}