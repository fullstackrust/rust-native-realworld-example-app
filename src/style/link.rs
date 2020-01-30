use iced::{button, Color};

pub enum Link {
  Nav { selected: bool },
}

impl button::StyleSheet for Link {
  fn active(&self) -> button::Style {
    match self {
      Link::Nav { selected } => {
        if *selected {
          button::Style {
            text_color: Color::from_rgb(0.10, 0.10, 0.10),
            ..button::Style::default()
          }
        } else {
          button::Style {
            text_color: Color::from_rgb(0.84, 0.84, 0.84),
            ..button::Style::default()
          }
        }
      }
    }
  }

  fn hovered(&self) -> button::Style {
    match self {
      Link::Nav { selected } => {
        if *selected {
          button::Style {
            text_color: Color::from_rgb(0.10, 0.10, 0.10),
            ..button::Style::default()
          }
        } else {
          button::Style {
            text_color: Color::from_rgb(0.40, 0.40, 0.40),
            ..button::Style::default()
          }
        }
      }
    }
  }
}
