use paat_core::types::event::EventMap;
use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::KeyModifiers;
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
use tuirealm::{AttrValue, Attribute, State, StateValue};
// tui
use crate::localization::fl;
use crate::messages::Message;
use crate::ports::ApiEvent;

use super::close_event_matcher;

#[derive(MockComponent)]
pub struct SelectFerry {
    component: List,
}

impl SelectFerry {
    pub fn build_table_rows(events: EventMap) -> (Attribute, AttrValue) {
        let mut events = events
            .values()
            .collect::<Vec<&paat_core::types::event::Event>>();
        events.sort_by_key(|event| event.start.clone());
        let mut builder = TableBuilder::default();
        for event in events {
            builder
                .add_col(TextSpan::from(format!("{}", event)))
                .add_row();
        }
        let table_rows = builder.build();
        (Attribute::Content, AttrValue::Table(table_rows))
    }
}

impl Default for SelectFerry {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .title(fl!("select-date-first"), Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str("🚀")
                .rewind(true)
                .step(4)
                .rows(TableBuilder::default().build())
                .selected_line(0),
        }
    }
}

impl Component<Message, ApiEvent> for SelectFerry {
    fn on(&mut self, event: Event<ApiEvent>) -> Option<Message> {
        if let Some(message) = close_event_matcher(event.clone(), |_| None) {
            return Some(message);
        }

        let command = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Down),
            Event::User(ApiEvent::FetchedEvents(events)) => {
                return Some(Message::EventsReceived(events));
            }
            _ => Cmd::None,
        };

        match self.perform(command) {
            CmdResult::Changed(State::One(StateValue::Usize(line_index))) => {
                Some(Message::FerryChanged(line_index))
            }
            _ => None,
        }
    }
}
