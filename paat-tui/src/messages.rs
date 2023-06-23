use paat_core::types::event::{EventMap, WaitForSpot};

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    AppClose,
    DepartureDateChanged(String),
    DepartureDateSubmitted(String),
    EventsReceived(EventMap),
    WaitResultReceived((String, WaitForSpot)),
    FerryChanged(usize),
    FerrySubmitted,
    LineChanged(usize),
    LineSubmitted,
    TickFromListener,
}
