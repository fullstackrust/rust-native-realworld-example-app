// // Input handler for different key commands and mouse button input.
// use iced_native::{Clipboard, Element, Event, Layout, Point, Widget};
// use iced_wgpu::{Defaults, Primitive, Renderer};

// use crate::message::Message;

// pub struct Input {}

// impl Input {
//   pub fn new() -> Self {
//     Self {}
//   }
// }

// impl<Message> Widget<Message, Renderer> for Input {
//   fn on_event(
//     &mut self,
//     event: Event,
//     layout: Layout<'_>,
//     cursor_position: Point,
//     messages: &mut Vec<Message>,
//     renderer: &Renderer,
//     clipboard: Option<&dyn Clipboard>,
//   ) {
//     match event {
//       Event::Keyboard(keyboard::Event::Input {
//         key_code,
//         state: ButtonState::Released,
//         ..
//       }) => match key_code {
//         keyboard::KeyCode::LeftArrow => {
//           self.state.is_pasting = None;
//         }
//         _ => {}
//       },
//       _ => {}
//     }
//   }
// }

// impl<'a, Message> Into<Element<'a, Message, Renderer>> for Input {
//   fn into(self) -> Element<'a, Message, Renderer> {
//     Element::new(self)
//   }
// }

// mod platform {
//   use crate::input::keyboard;

//   pub fn is_jump_modifier_pressed(modifiers: keyboard::ModifiersState) -> bool {
//     if cfg!(target_os = "macos") {
//       modifiers.alt
//     } else {
//       modifiers.control
//     }
//   }

//   pub fn is_copy_paste_modifier_pressed(modifiers: keyboard::ModifiersState) -> bool {
//     if cfg!(target_os = "macos") {
//       modifiers.logo
//     } else {
//       modifiers.control
//     }
//   }
// }
