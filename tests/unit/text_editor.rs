#[cfg(test)]
mod tests {
    use tui_coach::ui::text_editor::TextEditor;

    #[test]
    fn test_editor_creation() {
        let editor = TextEditor::new("Hello".to_string());
        assert_eq!(editor.content(), "Hello");
        assert!(editor.is_insert_mode());
    }

    #[test]
    fn test_insert_char() {
        let mut editor = TextEditor::new(String::new());
        editor.insert_char('H');
        editor.insert_char('i');
        assert_eq!(editor.content(), "Hi");
    }

    #[test]
    fn test_backspace() {
        let mut editor = TextEditor::new("Hi".to_string());
        editor.move_to_end();
        editor.backspace();
        assert_eq!(editor.content(), "H");
    }

    #[test]
    fn test_move_cursor() {
        let mut editor = TextEditor::new("Hello".to_string());
        editor.move_to_start();
        assert_eq!(editor.cursor_pos(), 0);
        editor.move_to_end();
        assert_eq!(editor.cursor_pos(), 5);
        editor.move_left();
        assert_eq!(editor.cursor_pos(), 4);
        editor.move_right();
        assert_eq!(editor.cursor_pos(), 5);
    }

    #[test]
    fn test_delete_char() {
        let mut editor = TextEditor::new("Hello".to_string());
        editor.move_to_start();
        editor.delete_char();
        assert_eq!(editor.content(), "ello");
    }

    #[test]
    fn test_delete_to_end() {
        let mut editor = TextEditor::new("Hello World".to_string());
        editor.move_to_end();
        editor.move_left();
        editor.move_left();
        editor.move_left();
        editor.move_left();
        editor.move_left();
        editor.move_left(); // Now at position 5
        editor.delete_to_end();
        assert_eq!(editor.content(), "Hello");
    }

    #[test]
    fn test_toggle_mode() {
        let mut editor = TextEditor::new("test".to_string());
        assert!(editor.is_insert_mode());
        editor.toggle_mode();
        assert!(!editor.is_insert_mode());
        editor.toggle_mode();
        assert!(editor.is_insert_mode());
    }
}
