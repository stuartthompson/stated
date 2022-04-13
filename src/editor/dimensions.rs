pub struct Dimensions {
    pub columns: u16,
    pub rows: u16,
}

impl Dimensions {
    pub fn default() -> Dimensions {
        Dimensions {
            columns: 80,
            rows: 24
        }
    }

    pub fn new(columns: u16, rows: u16) -> Dimensions {
        Dimensions {
            columns: columns,
            rows: rows
        }
    }

    pub fn from(dimensions: &Dimensions) -> Dimensions {
        Dimensions {
            columns: dimensions.columns,
            rows: dimensions.rows
        }
    }
}