use iced::{Button, Color, Column, HorizontalAlignment, Text};

use crate::message::Message;

pub fn new<'a>() -> Column<'a, Message> {
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
        // Column::new().padding(10).spacing(5).push(data.iter().fold(
        //     Column::new().padding(5).spacing(10),
        //     |choices, language| {
        //         choices.push(
        //             Button::new(
        //             state,
        //             Text::new(language)
        //                 .color(Color::WHITE)
        //                 .horizontal_alignment(HorizontalAlignment::Center),
        //         )
        //         .padding(12)
        //         //.border_radius(12)
        //         .min_width(100))
        //     },
        // ))
        Column::new().push(Text::new("home"))
    }

    let email_row = new_column(tag_data, true);

    Column::new().push(Text::new("Home Page"))
}
