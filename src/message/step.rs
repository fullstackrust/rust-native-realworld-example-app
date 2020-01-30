#[derive(Debug, Clone)]
pub enum StepMessage {
  InputChanged(String),
  ToggleSecureInput(bool),
}
