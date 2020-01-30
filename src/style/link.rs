use iced::{button, Background, Color, Vector};

pub enum Link {
  Nav { selected: bool },
}

impl button::StyleSheet for Link {
  fn active(&self) -> button::Style {
    match self {
      Link::Nav { selected } => {
        if *selected {
          button::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.7))),
            border_radius: 10,
            text_color: Color::WHITE,
            ..button::Style::default()
          }
        } else {
          button::Style::default()
        }
      }
    }
  }

  fn hovered(&self) -> button::Style {
    let active = self.active();

    button::Style {
      text_color: match self {
        Link::Nav { selected } if !selected => Color::from_rgb(0.2, 0.2, 0.7),
        _ => active.text_color,
      },
      shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
      ..active
    }
  }
}
