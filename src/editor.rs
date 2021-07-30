mod dimensions;

pub use dimensions::Dimensions;

/// An editor hosts a single open document. The program itself may have many
///   open editors. Each editor is given a different portion of the screen into
///   which it can render its content.
pub struct Editor {
    /// The dimensions allocated to this editor to use to display its contents.
    pub dimensions: Dimensions,
    
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
    /// Returns a new Editor.
    /// 
    /// # Arguments
    /// 
    /// * `columns` - The number of
    pub fn new(dimensions: Dimensions) -> Editor {
        Editor {
            dimensions: dimensions,
            column_ix: 0,
            row_ix: 0,
        }
    }

    /// Resizes the render area for an editor.
    ///
    /// # Arguments
    /// 
    /// * `self` - The editor being resized.
    /// * `dimensions` - The dimensions describing the updated render area.
    pub fn resize(&mut self, dimensions: Dimensions) {
        self.dimensions = dimensions;
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
        if self.column_ix + num_columns > self.dimensions.columns {
            // Ensure cursor remains within editor bounds.
            // TODO: Handle horizontal scrolling of document if line is wider than editor.
            self.column_ix = self.dimensions.columns - 1;
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
        if self.row_ix + num_rows > self.dimensions.rows {
            // Ensure cursor remains within editor bounds.
            self.row_ix = self.dimensions.rows - 1;
        } else {
            self.row_ix += num_rows;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the editor moves the cursor to the left.
    /// 
    /// Does not attempt to move the cursor outside the render boundary.
    #[test]
    fn moves_cursor_left_within_bounds() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Move the cursor 10 columns right, then 1 column to the left
        editor.move_cursor_right(10);
        editor.move_cursor_left(1);

        // Verify that the cursor moved as expected
        assert_eq!(editor.column_ix, 9);
    }

    /// Tests that the editor moves the cursor to the right.
    /// 
    /// Does not attempt to move the cursor outside the render boundary.
    #[test]
    fn moves_cursor_right_within_bounds() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Move cursor one column to the right
        editor.move_cursor_right(1);

        // Verify that the cursor moved as expected
        assert_eq!(editor.column_ix, 1);
    }

    /// Tests that the editor moves the cursor up.
    /// 
    /// Does not attempt to move the cursor outside the render boundary.
    #[test]
    fn moves_cursor_up_within_bounds() {
        let mut editor = Editor::new(Dimensions::new(80,24));

        // Move the cursor down 3 rows then up 1 row
        editor.move_cursor_down(3);
        editor.move_cursor_up(1);

        // Verify that the cursor moved as expected
        assert_eq!(editor.row_ix, 2);
    }

    /// Tests that the editor moves the cursor down.
    /// 
    /// Does not attempt to move the cursor outside the render boundary.
    #[test]
    fn moves_cursor_down_within_bounds() {
        let mut editor = Editor::new(Dimensions::new(80,24));

        // Move the cursor down 1 row
        editor.move_cursor_down(1);

        // Verify that the cursor moved as expected
        assert_eq!(editor.row_ix, 1);
    }

    /// Cursor should be constrained by the left side of the render boundary.
    #[test]
    fn cursor_constrained_within_editor_left_side() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor left (should constrain to 0)
        editor.move_cursor_left(1);

        assert_eq!(editor.column_ix, 0);
    }

    /// Cursor should be constrained by the right side of the render boundary.
    #[test]
    fn constrains_cursor_within_editor_right_side() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor right (should constrain to editor width)
        editor.move_cursor_right(100);

        assert_eq!(editor.column_ix, 79);
    }

    /// Cursor should be constrained by the top of the render boundary.
    #[test]
    fn cursor_constrained_within_editor_top() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor up (should constrain to 0)
        editor.move_cursor_up(1);

        assert_eq!(editor.row_ix, 0);
    }

    /// Cursor should be constrained by the bottom of the render boundary.
    #[test]
    fn constrains_cursor_within_editor_bottom() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor right (should constrain to editor height)
        editor.move_cursor_down(30);

        assert_eq!(editor.row_ix, 23);
    }
}