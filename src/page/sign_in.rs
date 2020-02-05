use iced::{Column, Text};

use crate::message::Message;

pub fn new<'a>() -> Column<'a, Message> {
    Column::new().push(Text::new("Sign In Page"))
}
