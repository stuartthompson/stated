pub struct Location {
    /// The index of the column described by this location.
    ///
    /// Zero-based. The 0th column is the left-hand most column.
    pub column_ix: u16,

    /// The index of the row described by this location.
    ///
    /// Zero-based. The 0th row is the top row in the editor.
    pub row_ix: u16,
}

impl Location {
    pub fn default() -> Location {
        Location { column_ix: 0, row_ix: 0 }
    }

    pub fn new(column_ix: u16, row_ix: u16) -> Location {
        Location { column_ix: column_ix, row_ix: row_ix }
    }
}