use iced::{button, Background, Color};

pub enum Link {
    Nav { selected: bool },
    Brand,
}

impl button::StyleSheet for Link {
    fn active(&self) -> button::Style {
        match self {
            Link::Nav { selected } => {
                if *selected {
                    button::Style {
                        text_color: Color::from_rgb(0.20, 0.20, 0.20),
                        ..button::Style::default()
                    }
                } else {
                    button::Style {
                        text_color: Color::from_rgb(0.70, 0.70, 0.70),
                        ..button::Style::default()
                    }
                }
            }
            Link::Brand => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.361, 0.722, 0.361))),
                border_radius: 0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Link::Nav { selected } => {
                if *selected {
                    button::Style {
                        text_color: Color::from_rgb(0.20, 0.20, 0.20),
                        ..button::Style::default()
                    }
                } else {
                    button::Style {
                        text_color: Color::from_rgb(0.40, 0.40, 0.40),
                        ..button::Style::default()
                    }
                }
            }
            Link::Brand => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.361, 0.722, 0.361))),
                border_radius: 0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
        }
    }
}
