use super::{Session, Subscription};

/// Session combined with subscription details (for player views)
#[derive(Debug, Clone)]
pub struct SessionWithSubscription {
    pub session: Session,
    pub subscription: Option<Subscription>,
}

impl SessionWithSubscription {
    pub fn new(session: Session, subscription: Option<Subscription>) -> Self {
        Self { session, subscription }
    }

    pub fn is_subscribed(&self) -> bool {
        self.subscription.is_some()
    }

    pub fn is_completed(&self) -> bool {
        self.subscription
            .as_ref()
            .and_then(|s| s.completed_at)
            .is_some()
    }

    pub fn subscription_id(&self) -> Option<i64> {
        self.subscription.as_ref().map(|s| s.id)
    }
}