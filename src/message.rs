use crate::page::Page;

#[derive(Debug, Clone)]
pub enum Message {
    PageChanged(Page),
    NoOp(),
}
