use super::{close_event_matcher, mocks::Calendar};
use crate::{localization::fl, messages::Message};
use paat_core::datetime::{get_current_date, naive_date_to_output_string};
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

#[derive(MockComponent)]
pub struct DepartureDate {
    component: Calendar,
}

impl DepartureDate {
    pub fn new() -> Self {
        let current_date = get_current_date();
        Self {
            component: Calendar::default()
                .value(naive_date_to_output_string(&current_date))
                .calendar_title(fl!("departure-date")),
        }
    }
}

impl Component<Message, NoUserEvent> for DepartureDate {
    fn on(&mut self, event: tuirealm::Event<NoUserEvent>) -> Option<Message> {
        if let Some(message) = close_event_matcher(event.clone(), |_| None) {
            return Some(message);
        }

        let command = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Down),
            _ => Cmd::None,
        };

        match self.perform(command) {
            CmdResult::Changed(State::One(StateValue::String(departure_date))) => {
                Some(Message::DepartureDateChanged(departure_date))
            }
            _ => None,
        }
    }
}
