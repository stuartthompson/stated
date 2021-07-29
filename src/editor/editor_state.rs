/// Represents the state of the editor.
pub struct EditorState {
    /// The number of columns this editor is currently allocated for display.
    pub columns: u16,
    /// The number of rows this editor is currently allocated for display.
    pub rows: u16,
    
    /// The index of the column where the cursor is currently located.
    ///
    /// Zero-based. The 0th column is the left-hand most column.
    pub column_ix: u16,

    /// The index of the row where the cursor is currently located.
    ///
    /// Zero-based. The 0th row is the top row in the editor.
    pub row_ix: u16,
}

impl EditorState {
    pub fn default() -> EditorState {
        EditorState {
            columns: 0,
            rows: 0,
            column_ix: 0,
            row_ix: 0,
        }
    }
}