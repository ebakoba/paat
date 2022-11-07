use paat_core::types::event::EventMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    AppClose,
    DepartureDateChanged(String),
    DepartureDateSubmitted(String),
    EventsReceived(EventMap),
    FerryChanged(usize),
    FerrySubmitted,
    LineChanged(usize),
    LineSubmitted,
}
