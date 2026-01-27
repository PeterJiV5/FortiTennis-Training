#[derive(Debug, Clone, PartialEq)]
pub enum SessionFilter {
    MySubscriptions,
    AllAvailable,
}

impl SessionFilter {
    pub fn toggle(&self) -> Self {
        match self {
            SessionFilter::MySubscriptions => SessionFilter::AllAvailable,
            SessionFilter::AllAvailable => SessionFilter::MySubscriptions,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SessionFilter::MySubscriptions => "My Subscriptions",
            SessionFilter::AllAvailable => "All Available",
        }
    }
}