use iced::{button, Button, Column, Font, HorizontalAlignment, Length, Row, Text};

use crate::message::Message;
use crate::page::Page;
use crate::style;

#[derive(Debug, Default, Clone)]
pub struct Nav {
    home_button: button::State,
    sign_in_button: button::State,
    sign_up_button: button::State,
    brand_button: button::State,
}

impl Nav {
    pub fn new() -> Nav {
        Nav {
            home_button: button::State::new(),
            sign_in_button: button::State::new(),
            sign_up_button: button::State::new(),
            brand_button: button::State::new(),
        }
    }

    pub fn view(&mut self, current_page: Page) -> Column<Message> {
        let Nav {
            home_button,
            sign_in_button,
            sign_up_button,
            brand_button,
        } = self;

        let nav_button = |state, label, link_page, current_page| {
            let label = Text::new(label).size(14);
            let button = Button::new(state, label).style(style::link::Link::Nav {
                selected: link_page == current_page,
            });

            button.on_press(Message::PageChanged(link_page)).padding(6)
        };

        // Font
        const BRAND_FONT: Font = Font::External {
            name: "TitilliumWeb-Bold",
            bytes: include_bytes!("../resources/TitilliumWeb-Bold.ttf"),
        };
        Column::new()
            .push(
                Row::new()
                    .push(
                        Column::new().spacing(10).push(
                            Text::new("conduit")
                                .color([0.361, 0.722, 0.361])
                                .font(BRAND_FONT)
                                .size(24), //.horizontal_alignment(HorizontalAlignment::Center),
                        ),
                    )
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(nav_button(home_button, "Home", Page::Home, current_page))
                            .push(nav_button(
                                sign_in_button,
                                "Sign in",
                                Page::SignIn,
                                current_page,
                            ))
                            .push(nav_button(
                                sign_up_button,
                                "Sign up",
                                Page::SignUp,
                                current_page,
                            )),
                    )
                    .spacing(250),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(
                            brand_button,
                            Row::new().spacing(10).spacing(250).push(
                                Text::new("conduit")
                                    .color([1.0, 1.0, 1.0])
                                    .font(BRAND_FONT)
                                    .size(72)
                                    .width(Length::Fill)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            ),
                        )
                        .width(Length::Fill)
                        .min_height(169) // 105px + 32px padding-top & 32px padding-bottom
                        // .padding(32)
                        .on_press(Message::NoOp())
                        .style(style::link::Link::Brand),
                    )
                    .spacing(250),
            )
    }
}
