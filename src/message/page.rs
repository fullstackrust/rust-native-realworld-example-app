#[derive(Debug, Clone)]
pub enum PageMessage {
  InputChanged(String),
  ToggleSecureInput(bool),
}
