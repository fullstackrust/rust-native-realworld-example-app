use crate::message::Message;
use iced::{Column, Element, Row, Text};

pub fn new<'a>() -> Element<'a, Message> {
    Row::new()
        .push(
            Column::new()
                .push(Text::new("FeedSelector"))
                .push(Text::new("Feed")),
        )
        .push(Column::new().push(Text::new("Global Feed")))
        .into()
}
