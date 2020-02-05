use iced::{button, Button, Color, Column, HorizontalAlignment, Text};
use crate::message::Message;
use crate::page::Page;

#[derive(Debug, Default, Clone)]
pub struct Tag {
    tag: String
    tag_button: button::State,
}

impl Tag {
    pub fn new(tag: String) -> Tag {
        Tag {
            tag,
            tag_button: button::State::new(),
        }
    }

    pub fn view(&mut self, current_page: Page) -> Column<Message> {
        let Tag { tag_button, tag } = self;

                            Button::new(
                                &mut tag_button,
                                Text::new(label.to_string())
                                    .color(Color::WHITE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .padding(12)
                            //.border_radius(12)
                            .min_width(100)
    }
}
