/// An editor hosts a single open document. The program itself may have many
///   open editors. Each editor is given a different portion of the screen into
///   which it can render its content.
pub struct Editor {
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

impl Editor {
    pub fn new() -> Editor {
        Editor {
            columns: 0,
            rows: 0,
            column_ix: 0,
            row_ix: 0,
        }
    }   
    pub fn resize(&mut self, columns: u16, rows: u16) {
        self.columns = columns;
        self.rows = rows;
    }

    /// Moves the cursor to the left a specified number of columns.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_columns` - The number of columns to move the cursor left.
    pub fn move_cursor_left(&mut self, num_columns: u16) {
        if num_columns > self.column_ix {
            // Ensure cursor remains within editor bounds.
            self.column_ix = 0;
        } else {
            self.column_ix -= num_columns;
        }
    }

    /// Moves the cursor to the right a specified number of columns.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_columns` - The number of columns to move the cursor right.
    pub fn move_cursor_right(&mut self, num_columns: u16) {
        if self.column_ix + num_columns > self.columns {
            // Ensure cursor remains within editor bounds.
            // TODO: Handle horizontal scrolling of document if line is wider than editor.
            self.column_ix = self.columns - 1;
        } else {
            self.column_ix += num_columns;
        }
    }

    /// Moves the cursor up a specified number of rows.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_rows` - The number of rows to move the cursor up.
    pub fn move_cursor_up(&mut self, num_rows: u16) {
        if num_rows > self.row_ix {
            // Ensure cursor remains within editor bounds.
            self.row_ix = 0;
        } else {
            self.row_ix -= num_rows;
        }
    }

    /// Moves the cursor down a specified number of rows.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_rows` - The number of rows to move the cursor down.
    pub fn move_cursor_down(&mut self, num_rows: u16) {
        if self.row_ix + num_rows > self.rows {
            // Ensure cursor remains within editor bounds.
            self.row_ix = self.rows - 1;
        } else {
            self.row_ix += num_rows;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the editor moves the cursor left one column when instructed.
    #[test]
    fn moves_cursor_left_one_column() {
        let mut editor = Editor::new();

        // Set the dimensions of this editor
        editor.resize(80, 24);

        // Move the cursor 10 columns right
        editor.move_cursor_right(10);

        // Move cursor one column to the left
        editor.move_cursor_left(1);

        // Verify that the cursor position moved as expected
        assert_eq!(editor.column_ix, 9);
    }

    /// Tests that the editor moves the cursor right one column when instructed.
    #[test]
    fn moves_cursor_right_one_column() {
        let mut editor = Editor::new();

        // Set the dimensions of this editor
        editor.resize(80, 24);

        // Move cursor one column to the right
        editor.move_cursor_right(1);

        // Verify that the cursor position moved as expected
        assert_eq!(editor.column_ix, 1);
    }

    #[test]
    fn constrains_cursor_within_editor_left_side() {
        let mut editor = Editor::new();

        // Set the dimensions of the editor
        editor.resize(80, 24);

        // Attempt to move the cursor left (should constrain to 0)
        editor.move_cursor_left(1);

        assert_eq!(editor.column_ix, 0);
    }

    #[test]
    fn constrains_cursor_within_editor_right_side() {
        let mut editor = Editor::new();

        // Set the dimensions of the editor
        editor.resize(80, 24);

        // Attempt to move the cursor right (should constrain to editor width)
        editor.move_cursor_right(100);

        assert_eq!(editor.column_ix, 79);
    }
}