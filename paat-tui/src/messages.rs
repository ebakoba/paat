#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    AppClose,
    DepartureDateChanged(String),
    DepartureDateSubmitted(String),
    LineChanged(usize),
    LineSubmitted(usize),
}
