/// Help screen and keyboard command reference
use crate::auth::UserContext;
use crate::ui::navigation::Screen;

pub struct HelpScreen;

impl HelpScreen {
    /// Get all available commands for the current screen
    pub fn get_commands(screen: &Screen, user_context: &UserContext) -> Vec<(&'static str, &'static str)> {
        match screen {
            Screen::Home => vec![
                ("[1]", "Go to Home"),
                ("[2]", "Go to Session Management"),
                ("[q]", "Quit"),
                ("[?]", "Show this help"),
            ],
            Screen::SessionList => {
                if user_context.is_coach() {
                    vec![
                        ("[c]", "Create new session"),
                        ("[e]", "Edit selected session"),
                        ("[d]", "Delete selected session"),
                        ("[↑↓]", "Navigate sessions"),
                        ("[Enter]", "View session details"),
                        ("[1]", "Go to Home"),
                        ("[q]", "Go to Home"),
                        ("[?]", "Show this help"),
                    ]
                } else {
                    vec![
                        ("[s]", "Subscribe/Unsubscribe"),
                        ("[f]", "Toggle filter (My/All)"),
                        ("[↑↓]", "Navigate sessions"),
                        ("[Enter]", "View session details"),
                        ("[1]", "Go to Home"),
                        ("[q]", "Go to Home"),
                        ("[?]", "Show this help"),
                    ]
                }
            }
            Screen::SessionDetail(_) => {
                if user_context.is_coach() {
                    vec![
                        ("[e]", "Edit this session"),
                        ("[d]", "Delete this session"),
                        ("[t]", "Manage training content"),
                        ("[1]", "Go to Home"),
                        ("[2]", "Back to Session List"),
                        ("[q]", "Go to Home"),
                        ("[?]", "Show this help"),
                    ]
                } else {
                    vec![
                        ("[s]", "Subscribe/Unsubscribe"),
                        ("[m]", "Mark as completed"),
                        ("[1]", "Go to Home"),
                        ("[2]", "Back to Session List"),
                        ("[q]", "Go to Home"),
                        ("[?]", "Show this help"),
                    ]
                }
            }
            Screen::SessionCreate | Screen::SessionEdit(_) => vec![
                ("[Tab]", "Next field"),
                ("[Shift+Tab]", "Previous field"),
                ("[Enter]", "Save changes"),
                ("[Esc]", "Cancel"),
                ("[?]", "Show this help"),
            ],
            Screen::SessionDelete(_) => vec![
                ("[y]", "Confirm deletion"),
                ("[n]", "Cancel deletion"),
                ("[Esc]", "Cancel deletion"),
                ("[?]", "Show this help"),
            ],
            Screen::TrainingContentCreate(_) | Screen::TrainingContentEdit(_) => vec![
                ("[Tab]", "Next field"),
                ("[Shift+Tab]", "Previous field"),
                ("[← →]", "Cycle content type"),
                ("[Enter]", "Save content"),
                ("[Esc]", "Cancel"),
                ("[?]", "Show this help"),
            ],
            Screen::TrainingContentDelete(_) => vec![
                ("[y]", "Confirm deletion"),
                ("[n]", "Cancel deletion"),
                ("[Esc]", "Cancel deletion"),
                ("[?]", "Show this help"),
            ],
            Screen::Help => vec![
                ("[q]", "Close help"),
                ("[Esc]", "Close help"),
                ("[↑↓]", "Scroll help text"),
                ("[Space]", "Page down"),
                ("[b]", "Page up"),
            ],
        }
    }

    /// Get brief footer help for current screen (3-4 most important commands)
    pub fn get_footer_help(screen: &Screen, user_context: &UserContext) -> Vec<(&'static str, &'static str)> {
        match screen {
            Screen::Home => vec![
                ("[2]", "Manage Sessions"),
                ("[?]", "Help"),
                ("[q]", "Quit"),
            ],
            Screen::SessionList => {
                if user_context.is_coach() {
                    vec![
                        ("[c]", "Create"),
                        ("[e]", "Edit"),
                        ("[d]", "Delete"),
                        ("[?]", "Help"),
                    ]
                } else {
                    vec![
                        ("[s]", "Subscribe"),
                        ("[f]", "Filter"),
                        ("[Enter]", "View"),
                        ("[?]", "Help"),
                    ]
                }
            }
            Screen::SessionDetail(_) => {
                if user_context.is_coach() {
                    vec![
                        ("[t]", "Training"),
                        ("[e]", "Edit"),
                        ("[2]", "Back"),
                    ]
                } else {
                    vec![
                        ("[m]", "Complete"),
                        ("[s]", "Subscribe"),
                        ("[2]", "Back"),
                    ]
                }
            }
            Screen::SessionCreate => vec![
                ("[Tab]", "Next"),
                ("[Enter]", "Save"),
                ("[Esc]", "Cancel"),
            ],
            Screen::SessionEdit(_) => vec![
                ("[Tab]", "Next"),
                ("[Enter]", "Save"),
                ("[Esc]", "Cancel"),
            ],
            Screen::SessionDelete(_) => vec![
                ("[y]", "Delete"),
                ("[n]", "Cancel"),
            ],
            Screen::TrainingContentCreate(_) | Screen::TrainingContentEdit(_) => vec![
                ("[Tab]", "Next"),
                ("[← →]", "Content type"),
                ("[Enter]", "Save"),
                ("[Esc]", "Cancel"),
            ],
            Screen::TrainingContentDelete(_) => vec![
                ("[y]", "Delete"),
                ("[n]", "Cancel"),
            ],
            Screen::Help => vec![
                ("[q]", "Close"),
                ("[↑↓]", "Scroll"),
            ],
        }
    }

    /// Get the full help text
    pub fn get_help_text() -> Vec<String> {
        vec![
            "═══════════════════════════════════════════════════════════════════".to_string(),
            "TENNIS TUI COACH - KEYBOARD COMMANDS REFERENCE".to_string(),
            "═══════════════════════════════════════════════════════════════════".to_string(),
            "".to_string(),
            "GLOBAL COMMANDS:".to_string(),
            "  [?]        Show this help screen".to_string(),
            "  [q]        Return to home screen (or quit if on home)".to_string(),
            "  [1]        Jump to home screen".to_string(),
            "  [Esc]      Go back to previous screen".to_string(),
            "".to_string(),
            "NAVIGATION COMMANDS:".to_string(),
            "  [2]        Go to Session Management".to_string(),
            "  [↑] [k]    Navigate up / Previous item (vi-style)".to_string(),
            "  [↓] [j]    Navigate down / Next item (vi-style)".to_string(),
            "  [Enter]    Select/View item or confirm action".to_string(),
            "".to_string(),
            "SESSION LIST COMMANDS (Coach):".to_string(),
            "  [c]        Create new session".to_string(),
            "  [e]        Edit selected session".to_string(),
            "  [d]        Delete selected session".to_string(),
            "".to_string(),
            "SESSION LIST COMMANDS (Player):".to_string(),
            "  [s]        Subscribe/Unsubscribe to session".to_string(),
            "  [f]        Toggle filter: My Sessions vs All Available".to_string(),
            "".to_string(),
            "SESSION DETAIL COMMANDS (Player):".to_string(),
            "  [m]        Mark session as completed".to_string(),
            "".to_string(),
            "SESSION DETAIL DISPLAY:".to_string(),
            "  View all session information including:".to_string(),
            "  - Session title, description, date, time, duration".to_string(),
            "  - Skill level and subscription status".to_string(),
            "  - Training content (drills, exercises, warmups, cooldowns)".to_string(),
            "  - Estimated duration from training activities".to_string(),
            "".to_string(),
            "FORM EDITING COMMANDS:".to_string(),
            "  [Tab]      Move to next field".to_string(),
            "  [Shift+Tab] Move to previous field".to_string(),
            "  [↑] [↓]    Move up/down between fields".to_string(),
            "  [← →]      Cycle skill level (when on skill level field)".to_string(),
            "  [Enter]    Save form".to_string(),
            "  [Esc]      Cancel without saving".to_string(),
            "".to_string(),
            "DELETION COMMANDS:".to_string(),
            "  [y]        Confirm deletion".to_string(),
            "  [n]        Cancel deletion".to_string(),
            "  [Esc]      Cancel deletion".to_string(),
            "".to_string(),
            "TEXT EDITOR (Vim-style):".to_string(),
            "  [i]        Enter insert mode (when in normal mode)".to_string(),
            "  [Esc]      Enter normal mode (when in insert mode)".to_string(),
            "  [h] [←]    Move cursor left".to_string(),
            "  [l] [→]    Move cursor right".to_string(),
            "  [0]        Move to start of line".to_string(),
            "  [$]        Move to end of line".to_string(),
            "  [x]        Delete character at cursor (normal mode)".to_string(),
            "  [d$]       Delete from cursor to end (normal mode)".to_string(),
            "  Insert text - Type normally in insert mode".to_string(),
            "  Backspace - Delete character before cursor (insert mode)".to_string(),
            "".to_string(),
            "FIELD TYPES:".to_string(),
            "  Title              Required: 3-100 characters".to_string(),
            "  Description        Optional: max 500 characters".to_string(),
            "  Date               Format: YYYY-MM-DD (optional)".to_string(),
            "  Time               Format: HH:MM (optional)".to_string(),
            "  Duration           Range: 5-480 minutes (optional)".to_string(),
            "  Skill Level        Choose: Beginner, Intermediate, or Advanced".to_string(),
            "".to_string(),
            "═══════════════════════════════════════════════════════════════════".to_string(),
            "Press [q] or [Esc] to close help and return to application".to_string(),
            "═══════════════════════════════════════════════════════════════════".to_string(),
        ]
    }
}
