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

    pub fn view(&mut self, current_page: Page) -> Column<Message> {
        let Home { sign_in_button } = self;
        //let state = sign_in_button;
        //let (var1, var2, var3) = functionreturningatuple();
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
        //(0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
        fn new_column(data: Vec<&str>, sign_in_button: button::State) -> Column<Message> {
            Column::new().padding(10).spacing(5).push(data.iter().fold(
                Column::new().padding(5).spacing(10),
                |choices, label| {
                    choices.push(
                        Button::new(
                            &mut sign_in_button,
                            Text::new(label.to_string())
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

        let email_row = new_column(tag_data, *sign_in_button);

        Column::new().push(Text::new("Home Page"))
    }
}
