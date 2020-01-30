pub mod page;

use page::PageMessage;

#[derive(Debug, Clone)]
pub enum Message {
  BackPressed,
  NextPressed,
  PageMessage(PageMessage),
}
