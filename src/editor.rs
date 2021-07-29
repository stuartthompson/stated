mod editor_state;

pub use editor_state::EditorState;
use crate::core_data::CoreData;

/// An editor hosts a single open document. The program itself may have many
///   open editors. Each editor is given a different portion of the screen into
///   which it can render its content.
pub struct Editor {
    editor_state: EditorState,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            editor_state: EditorState::default(),
        }
    }
    pub fn resize(&mut self, columns: u16, rows: u16) {
        self.editor_state.columns = columns;
        self.editor_state.rows = rows;
    }
    pub fn move_cursor(&mut self, delta_col: u16, delta_row: u16) {
        self.editor_state.column_ix += delta_col;
        self.editor_state.row_ix += delta_row;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Tests that the editor moves the cursor right one column when instructed.
    #[test]
    fn moves_cursor_right_one_column() {
        let mut editor = Editor::new();

        // Set the dimensions of this editor
        editor.resize(80, 24);

        // Move cursor one column to the right
        editor.move_cursor(1, 0);

        // Verify that the cursor position moved as expected
        assert_eq!(editor.editor_state.column_ix, 1);
    }
}