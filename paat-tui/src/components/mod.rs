mod departure_date;
mod header;
mod mocks;
mod select_ferry;
mod select_line;

use crate::messages::Message;
pub use departure_date::DepartureDate;
pub use header::AppHeader;
pub use select_ferry::SelectFerry;
pub use select_line::SelectLine;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    Event, NoUserEvent,
};

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentId {
    Header,
    DepartureDate,
    SelectFerry,
    SelectLine,
}

pub fn close_event_matcher<F>(event: Event<NoUserEvent>, rest_of_match: F) -> Option<Message>
where
    F: Fn(Event<NoUserEvent>) -> Option<Message>,
{
    match event {
        Event::Keyboard(KeyEvent {
            code: Key::Esc,
            modifiers: KeyModifiers::NONE,
        }) => Some(Message::AppClose),
        _ => rest_of_match(event),
    }
}
