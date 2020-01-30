use iced::{text_input, Checkbox, Column, Element, HorizontalAlignment, Length, Text, TextInput};

use crate::message::step::StepMessage;
// use crate::view::input::Input;

pub struct Steps {
  steps: Vec<Step>,
  current: usize,
}

impl Steps {
  pub fn new() -> Steps {
    Steps {
      steps: vec![
        Step::Welcome,
        Step::TextInput {
          value: String::new(),
          is_secure: false,
          state: text_input::State::new(),
        },
      ],
      current: 0,
    }
  }

  pub fn update(&mut self, msg: StepMessage) {
    self.steps[self.current].update(msg);
  }

  pub fn view(&mut self) -> Element<StepMessage> {
    // Input::new();
    self.steps[self.current].view()
  }

  pub fn advance(&mut self) {
    if self.can_continue() {
      self.current += 1;
    }
  }

  pub fn go_back(&mut self) {
    if self.has_previous() {
      self.current -= 1;
    }
  }

  pub fn has_previous(&self) -> bool {
    self.current > 0
  }

  pub fn can_continue(&self) -> bool {
    self.current + 1 < self.steps.len() && self.steps[self.current].can_continue()
  }

  pub fn title(&self) -> &str {
    self.steps[self.current].title()
  }
}

enum Step {
  Welcome,
  TextInput {
    value: String,
    is_secure: bool,
    state: text_input::State,
  },
}

impl<'a> Step {
  fn update(&mut self, msg: StepMessage) {
    match msg {
      StepMessage::InputChanged(new_value) => {
        if let Step::TextInput { value, .. } = self {
          *value = new_value;
        }
      }
      StepMessage::ToggleSecureInput(toggle) => {
        if let Step::TextInput { is_secure, .. } = self {
          *is_secure = toggle;
        }
      }
    };
  }

  fn title(&self) -> &str {
    match self {
      Step::Welcome => "Welcome",
      Step::TextInput { .. } => "Text input",
    }
  }

  fn can_continue(&self) -> bool {
    match self {
      Step::Welcome => true,
      Step::TextInput { value, .. } => !value.is_empty(),
    }
  }

  fn view(&mut self) -> Element<StepMessage> {
    match self {
      Step::Welcome => Self::welcome(),
      Step::TextInput {
        value,
        is_secure,
        state,
      } => Self::text_input(value, *is_secure, state),
    }
    .into()
  }

  fn container(title: &str) -> Column<'a, StepMessage> {
    Column::new().spacing(20).push(Text::new(title).size(50))
  }

  fn welcome() -> Column<'a, StepMessage> {
    Self::container("Welcome!").push(Text::new(
      "This is a simple Calculator \
                 that can be easily implemented on top of Iced.",
    ))
  }

  fn text_input(
    value: &str,
    is_secure: bool,
    state: &'a mut text_input::State,
  ) -> Column<'a, StepMessage> {
    let text_input = TextInput::new(
      state,
      "Type something to continue...",
      value,
      StepMessage::InputChanged,
    )
    .padding(10)
    .size(30);
    Self::container("Text input")
      .push(Text::new(
        "Use a text input to ask for different kinds of information.",
      ))
      .push(if is_secure {
        text_input.password()
      } else {
        text_input
      })
      .push(Checkbox::new(
        is_secure,
        "Enable password mode",
        StepMessage::ToggleSecureInput,
      ))
      .push(Text::new(
        "A text input produces a message every time it changes. It is \
                 very easy to keep track of its contents:",
      ))
      .push(
        Text::new(if value.is_empty() {
          "You have not typed anything yet..."
        } else {
          value
        })
        .width(Length::Fill)
        .horizontal_alignment(HorizontalAlignment::Center),
      )
  }
}
