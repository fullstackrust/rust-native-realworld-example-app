use crate::message::Message;
use iced::Row;
use serde::{Deserialize, Serialize};

pub mod home;
pub mod sign_in;
pub mod sign_up;

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

pub fn view<'a>(page: Page) -> Row<'a, Message> {
    let page = match page {
        Page::Home => home::new(),
        Page::SignIn => sign_in::new(),
        Page::SignUp => sign_up::new(),
    };

    Row::new().push(page)
}
