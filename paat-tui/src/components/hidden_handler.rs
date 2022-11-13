use crate::{messages::Message, ports::ApiEvent};
use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    Component, Event, MockComponent,
};

#[derive(MockComponent, Default)]
pub struct HiddenHandler {
    component: Phantom,
}

impl Component<Message, ApiEvent> for HiddenHandler {
    fn on(&mut self, event: tuirealm::Event<ApiEvent>) -> Option<Message> {
        match event {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => Some(Message::AppClose),
            Event::User(ApiEvent::FetchedEvents(events)) => {
                return Some(Message::EventsReceived(events));
            }
            _ => None,
        }
    }
}
