#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    AppClose,
    DepartureDateChanged(String),
}
