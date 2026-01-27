use crate::auth::UserContext;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    SessionList,
    SessionDetail(i64), // session_id
    SessionCreate,
    SessionEdit(i64),   // session_id
    SessionDelete(i64), // session_id
}

impl Screen {
    /// Get available menu items based on user role
    pub fn get_menu_items(user_context: &UserContext) -> Vec<String> {
        if user_context.is_coach() {
            vec![
                ("Home", Screen::Home),
                ("Manage Session", Screen::SessionList),
            ]
        } else {
            vec![("Home", Screen::Home), ("My Session", Screen::SessionList)]
        }
    }
}
