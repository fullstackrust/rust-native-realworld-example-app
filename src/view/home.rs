use iced::{button, Button, Color, Column, HorizontalAlignment, Text};

use crate::message::Message;
use crate::page::Page;
//use crate::style;

#[derive(Debug, Default, Clone)]
pub struct Home {
    sign_in_button: button::State,
}

impl Home {
    pub fn new() -> Home {
        Home {
            sign_in_button: button::State::new(),
        }
    }

    pub fn view(&mut self, Home:sign_in_button, current_page: Page) -> Column<Message> {
        let Home { sign_in_button } = self;
        let state = Home: sign_in_button;
        let tag_data = vec![
            "but",
            "test",
            "dragons",
            "training",
            "tags",
            "as",
            "coffee",
            "animation",
            "baby",
            "flowers",
            "money",
            "caramel",
            "cars",
            "japan",
            "tag1",
            "happiness",
            "sugar",
            "clean",
            "cookies",
            "sushi",
        ];

        // reduced to newColumn
        fn new_column(data: Vec<&str>, is_secure: bool) -> Column<Message> {
            Column::new().padding(10).spacing(5).push(data.iter().fold(
                Column::new().padding(5).spacing(10),
                |choices, language| {
                    choices.push(
                        Button::new(
                            state,
                            Text::new(language.to_string())
                                .color(Color::WHITE)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .padding(12)
                        //.border_radius(12)
                        .min_width(100),
                    )
                },
            ))
            //Column::new().push(Text::new("home"))
        }

        let email_row = new_column(tag_data, true);

        Column::new().push(Text::new("Home Page"))
    }
}
