use crate::page::Page;

#[derive(Debug, Clone)]
pub enum PageMessage {
  PageChanged(Page),
}
