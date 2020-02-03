use iced::{
    executor, scrollable, Application, Column, Command, Container, Element, Length, Row,
    Scrollable, Settings,
};

mod data;
mod message;
mod page;
mod style;
mod view;

use crate::message::Message;
use crate::page::Page;
use crate::view::nav::Nav;

pub fn main() {
    env_logger::init();

    Conduit::run(Settings::default())
}

pub struct Conduit {
    page: Page,
    nav: Nav,
    scroll: scrollable::State,
}

impl Application for Conduit {
    type Executor = executor::Default;
    type Message = Message;

    fn new() -> (Conduit, Command<Message>) {
        (
            Conduit {
                page: Page::default(),
                nav: Nav::new(),
                scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        match event {
            Message::PageChanged(page) => {
                self.page = page;
            }
            Message::NoOp() => {}
        }

        Command::none()
    }

    fn title(&self) -> String {
        "Conduit".to_owned()
    }

    fn view(&mut self) -> Element<Message> {
        let content: Element<_> = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .padding(10)
            .push(self.nav.view(self.page))
            .push(layout(self.page))
            .into();

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

fn layout<'a>(page: page::Page) -> Row<'a, Message> {
    let page = match page {
        Page::Home => view::home::new(),
        Page::SignIn => view::sign_in::new(),
        Page::SignUp => view::sign_up::new(),
    };

    Row::new().push(page)
}

// This should be gracefully handled by Iced in the future. Probably using our
// own proc macro, or maybe the whole process is streamlined by `wasm-pack` at
// some point.
#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn run() {
        super::main()
    }
}
