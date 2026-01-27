use crate::models::User;

/// Holds the currently authenticated user's context
#[derive(Debug, Clone)]
pub struct UserContext {
    pub user: User,
}

impl UserContext {
    pub fn new(user: User) -> Self {
        Self { user }
    }

    pub fn username(&self) -> &str {
        &self.user.username
    }

    pub fn is_coach(&self) -> bool {
        self.user.is_coach()
    }

    pub fn is_player(&self) -> bool {
        self.user.is_player()
    }
}
