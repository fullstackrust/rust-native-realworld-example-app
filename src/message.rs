pub mod step;

use step::StepMessage;

#[derive(Debug, Clone)]
pub enum Message {
  BackPressed,
  NextPressed,
  StepMessage(StepMessage),
}
