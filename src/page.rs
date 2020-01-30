use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Page {
    Home,
    SignIn,
    SignUp,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}
