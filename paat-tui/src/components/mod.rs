mod departure_date;
mod header;
mod mocks;

pub use departure_date::DepartureDate;
pub use header::AppHeader;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    Event, NoUserEvent,
};

use crate::messages::Message;

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ComponentId {
    Header,
    DepartureDate,
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
