/// Simple vim-like text editor for TUI form inputs
#[derive(Debug, Clone)]
pub struct TextEditor {
    content: String,
    cursor_pos: usize,
    insert_mode: bool,
}

impl TextEditor {
    pub fn new(content: String) -> Self {
        Self {
            cursor_pos: 0,
            insert_mode: true,
            content,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_insert_mode(&self) -> bool {
        self.insert_mode
    }

    pub fn toggle_mode(&mut self) {
        self.insert_mode = !self.insert_mode;
    }

    /// Handle character input in insert mode
    pub fn insert_char(&mut self, c: char) {
        if self.insert_mode && c.is_alphanumeric() || (c == ' ' || c == '-' || c == ':' || c == '/') {
            self.content.insert(self.cursor_pos, c);
            self.cursor_pos += c.len_utf8();
            if self.cursor_pos > self.content.len() {
                self.cursor_pos = self.content.len();
            }
        }
    }

    /// Handle backspace in insert mode
    pub fn backspace(&mut self) {
        if self.insert_mode && self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.content.remove(self.cursor_pos);
        }
    }

    /// Move cursor left (vim: h)
    pub fn move_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    /// Move cursor right (vim: l)
    pub fn move_right(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.cursor_pos += 1;
        }
    }

    /// Move to start of line (vim: 0)
    pub fn move_to_start(&mut self) {
        self.cursor_pos = 0;
    }

    /// Move to end of line (vim: $)
    pub fn move_to_end(&mut self) {
        self.cursor_pos = self.content.len();
    }

    /// Delete character at cursor (vim: x)
    pub fn delete_char(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.content.remove(self.cursor_pos);
        }
    }

    /// Delete from cursor to end of line (vim: D)
    pub fn delete_to_end(&mut self) {
        self.content.truncate(self.cursor_pos);
    }

    /// Delete all content (vim: ggdG)
    pub fn delete_all(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.cursor_pos = self.content.len();
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_pos = 0;
    }
}
