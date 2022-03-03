mod dimensions;
mod location;

pub use dimensions::Dimensions;
pub use location::Location;

use crate::document::TextDocument;

/// An editor hosts a single open document. The program itself may have many
///   open editors. Each editor is given a different portion of the screen into
///   which it can render its content.
pub struct Editor<'a> {
    /// The dimensions allocated to this editor to use to display its contents.
    pub dimensions: Dimensions,
    
    /// The location of the cursor in this editor.
    pub cursor_location: Location,

    /// The amount of scrolling (columns and rows) applied to the editor.
    pub scroll_amount: Location,

    /// The content of the document currently being displayed in this editor.
    pub content: Option<&'a str>
}

impl<'a> Editor<'a> {
    /// Returns a new Editor.
    /// 
    /// # Arguments
    /// 
    /// * `dimensions` - The dimensions of this editor.
    pub fn new(dimensions: Dimensions) -> Editor<'a> {
        Editor {
            dimensions: dimensions,
            cursor_location: Location::default(),
            scroll_amount: Location::default(),
            content: None
        }
    }

    /// Sets the content of an editor.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The editor into which to set content.
    /// * `document` - A document containing the contents to load.
    pub fn set_content(&mut self, document: &'a TextDocument) {
        self.content = Some(document.get_content());
    }

    /// Gets the content to render in this editor.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The editor for which to get render content.
    pub fn get_render_content(&self) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        if let Some(content) = self.content {
            let lines = content.split("\r\n");
            
            // Iterate available lines
            let mut ix = 0;
            for line in lines {
                // Only render this line if there is enough space in viewport
                if ix < self.dimensions.rows {
                    // Dimensions dictate how many columns are visible
                    let cols = self.dimensions.columns as usize;
                    // Determine what part of the line should be rendered
                    let start = self.scroll_amount.column_ix as usize;
                    let mut end = start + cols;
                    if end > line.len() {
                        end = line.len();
                    }
                    result.push(&line[start..end]);
                } else {
                    break;
                }
                ix += 1;
            }
        }
        result
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

    pub fn scroll_to(&mut self, column_ix: u16, row_ix: u16) {
        self.scroll_amount.column_ix = column_ix;
        self.scroll_amount.row_ix = row_ix;
    }

    /// Moves the cursor to the left a specified number of columns.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_columns` - The number of columns to move the cursor left.
    pub fn move_cursor_left(&mut self, num_columns: u16) {
        if num_columns > self.cursor_location.column_ix {
            // Ensure cursor remains within editor bounds.
            self.cursor_location.column_ix = 0;
        } else {
            self.cursor_location.column_ix -= num_columns;
        }
    }

    /// Moves the cursor to the right a specified number of columns.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_columns` - The number of columns to move the cursor right.
    pub fn move_cursor_right(&mut self, num_columns: u16) {
        if self.cursor_location.column_ix + num_columns > self.dimensions.columns - 1 {
            // Check if scroll position needs to be updated
            if let Some(content) = self.content {
                let mut lines = content.split("\r\n");
                let cur_row = self.cursor_location.row_ix as usize;
                if let Some(line) = &lines.nth(cur_row) {
                    let end = (self.scroll_amount.column_ix + self.dimensions.columns) as usize;
                    if line.len() > end {
                        // Increment scroll position
                        self.scroll_amount.column_ix += 1;
                    }
                }
            }
            // Ensure cursor remains within editor bounds.
            self.cursor_location.column_ix = self.dimensions.columns - 1;
        } else {
            self.cursor_location.column_ix += num_columns;
        }
    }

    /// Moves the cursor up a specified number of rows.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_rows` - The number of rows to move the cursor up.
    pub fn move_cursor_up(&mut self, num_rows: u16) {
        if num_rows > self.cursor_location.row_ix {
            // Ensure cursor remains within editor bounds.
            self.cursor_location.row_ix = 0;
        } else {
            self.cursor_location.row_ix -= num_rows;
        }
    }

    /// Moves the cursor down a specified number of rows.
    ///
    /// # Arguments
    ///
    /// * `self` - The editor in which to move the cursor.
    /// * `num_rows` - The number of rows to move the cursor down.
    pub fn move_cursor_down(&mut self, num_rows: u16) {
        if self.cursor_location.row_ix + num_rows > self.dimensions.rows {
            // Ensure cursor remains within editor bounds.
            self.cursor_location.row_ix = self.dimensions.rows - 1;
        } else {
            self.cursor_location.row_ix += num_rows;
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
        assert_eq!(editor.cursor_location.column_ix, 9);
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
        assert_eq!(editor.cursor_location.column_ix, 1);
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
        assert_eq!(editor.cursor_location.row_ix, 2);
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
        assert_eq!(editor.cursor_location.row_ix, 1);
    }

    /// Cursor should be constrained by the left side of the render boundary.
    #[test]
    fn cursor_constrained_within_editor_left_side() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor left (should constrain to 0)
        editor.move_cursor_left(1);

        assert_eq!(editor.cursor_location.column_ix, 0);
    }

    /// Cursor should be constrained by the right side of the render boundary.
    #[test]
    fn constrains_cursor_within_editor_right_side() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor right (should constrain to editor width)
        editor.move_cursor_right(100);

        assert_eq!(editor.cursor_location.column_ix, 79);
    }

    /// Cursor should be constrained by the top of the render boundary.
    #[test]
    fn cursor_constrained_within_editor_top() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor up (should constrain to 0)
        editor.move_cursor_up(1);

        assert_eq!(editor.cursor_location.row_ix, 0);
    }

    /// Cursor should be constrained by the bottom of the render boundary.
    #[test]
    fn constrains_cursor_within_editor_bottom() {
        let mut editor = Editor::new(Dimensions::new(80, 24));

        // Attempt to move the cursor right (should constrain to editor height)
        editor.move_cursor_down(30);

        assert_eq!(editor.cursor_location.row_ix, 23);
    }

    /// Gets render content for an empty document.
    #[test]
    fn get_render_content_when_editor_empty() {
        let editor = Editor::new(Dimensions::new(80, 24));

        assert_eq!(editor.get_render_content(), Vec::<&str>::new());
    }

    /// Gets the render content for a simple document.
    #[test]
    fn get_render_content() {
        let mut editor = Editor::new(Dimensions::new(80, 24));
        let document = TextDocument::new("Hello\r\nWorld!");
        editor.set_content(&document);

        assert_eq!(editor.get_render_content(), vec!["Hello", "World!"]);
    }

    /// Verifies that the editor trims content vertically when the content is 
    ///  too tall to fit.
    #[test]
    fn get_render_content_when_too_tall_to_fit() {
        let mut editor = Editor::new(Dimensions::new(10, 1));
        let document = TextDocument::new("First\r\nSecond");
        editor.set_content(&document);

        assert_eq!(editor.get_render_content(), vec!["First"]);
    }

    /// Verifies that the editor trims the rendered content when that content
    ///  is too wide to fit into the render area.
    #[test]
    fn get_render_content_when_too_wide_to_fit() {
        let mut editor = Editor::new(Dimensions::new(4, 1));
        let document = TextDocument::new("First");
        editor.set_content(&document);

        assert_eq!(editor.get_render_content(), vec!["Firs"]);
    }

    /// Tests that scrolling one character to the right causes the editor to 
    ///  return the correct render content.
    #[test]
    fn get_render_content_when_scrolled_width() {
        let mut editor = Editor::new(Dimensions::new(4, 1));
        let document = TextDocument::new("First");
        editor.set_content(&document);

        editor.scroll_to(1, 0);
        assert_eq!(editor.get_render_content(), vec!["irst"]);
    }
    
    /// Tests that scrolling beyond the end of the current content correctly 
    ///  loads the scrolled content into view.
    #[test]
    fn get_render_content_when_scrolled_beyond_end_of_content() {
        let mut editor = Editor::new(Dimensions::new(4, 1));
        let document = TextDocument::new("First");
        editor.set_content(&document);

        editor.scroll_to(2, 0);
        assert_eq!(editor.get_render_content(), vec!["rst"]);
    }

    /// Tests that the editor auto-scrolls when the cursor moves beyond the 
    ///  right of the current content.
    #[test]
    fn auto_scroll_when_cursor_moved_too_far_right() {
        let mut editor = Editor::new(Dimensions::new(4, 1));
        let document = TextDocument::new("First");
        editor.set_content(&document);

        // Move cursor 4 columns to the right (should force scroll)
        editor.move_cursor_right(4);
        assert_eq!(editor.get_render_content(), vec!["irst"]);
    }

    /// Tests that the editor auto-scrolls when the cursor moves beyond the 
    ///  right of the current content.
    /// 
    /// Deliberately scrolls far to many times to verify that the editor stops
    ///  auto-scrolling once the end of the content is reached.
    #[test]
    fn auto_scroll_when_cursor_moved_too_far_right_many_times() {
        let mut editor = Editor::new(Dimensions::new(4, 1));
        let document = TextDocument::new("First");
        editor.set_content(&document);

        // Attempt to move cursor right 10 times (should force scroll after 4)
        editor.move_cursor_right(10);
        assert_eq!(editor.get_render_content(), vec!["irst"]);
    }
}