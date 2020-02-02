use iced::{
    button, scrollable, Align, Button, Color, Column, Container, Element, Font,
    HorizontalAlignment, Length, Row, Scrollable, Text,
};

use crate::message::Message;
use crate::page::Page;
use crate::style;

#[derive(Debug, Default, Clone)]
pub struct Nav {
    home_button: button::State,
    sign_in_button: button::State,
    sign_up_button: button::State,
    scroll: scrollable::State,
    debug: bool,
}

impl Nav {
    pub fn new() -> Nav {
        Nav {
            home_button: button::State::new(),
            sign_in_button: button::State::new(),
            sign_up_button: button::State::new(),
            scroll: scrollable::State::new(),
            debug: false,
        }
    }

    pub fn view(&mut self, current_page: Page) -> Column<Message> {
        let Nav {
            home_button,
            sign_in_button,
            sign_up_button,
            scroll,
            debug,
        } = self;

        let nav_button = |state, label, link_page, current_page| {
            let label = Text::new(label).size(14);
            let button = Button::new(state, label).style(style::link::Link::Nav {
                selected: link_page == current_page,
            });

            button.on_press(Message::PageChanged(link_page)).padding(6)
        };

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            // .push(steps.view(self.debug).map(Message::StepMessage))
            // .push(controls)
            .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        let scrollable =
            Scrollable::new(scroll).push(Container::new(content).width(Length::Fill).center_x());

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
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(
                        Column::new().spacing(10).push(
                            Text::new("conduit")
                                .color([0.361, 0.722, 0.361])
                                .font(BRAND_FONT)
                                .size(48)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        ),
                    )
                    .spacing(0),
            )
            .push(
                Container::new(scrollable)
                    .height(Length::Fill)
                    .center_y()
                    .into(),
            )
    }
}
