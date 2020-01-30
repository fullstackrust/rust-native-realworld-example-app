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
            background: Some(Background::Color(Color::WHITE)), // 0, 0, 0, 0.8 is active black
            border_radius: 10,
            text_color: Color::from_rgb(0.10, 0.10, 0.10),
            ..button::Style::default()
          }
        } else {
          //button::Style::default()
          button::Style {
            text_color: Color::from_rgb(0.84, 0.84, 0.84),
            ..button::Style::default()
          }
        }
      }
    }
  }

  fn hovered(&self) -> button::Style {
    //let active = self.active();
    match self {
      Link::Nav { selected: _ } => {
        button::Style {
          // background: Some(Background::Color(Color::WHITE)), // 0, 0, 0, 0.8 is active black
          // border_radius: 10,
          text_color: Color::from_rgb(0.10, 0.10, 0.10),
          ..button::Style::default()
        }
      }
    }
  }
}
