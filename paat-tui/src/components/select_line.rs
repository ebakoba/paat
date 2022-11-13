use crate::localization::fl;
use crate::messages::Message;
use crate::ports::ApiEvent;
use paat_core::constants::LINES;
use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::event::KeyModifiers;
use tuirealm::props::{Alignment, BorderType, Borders, Color, Table, TableBuilder, TextSpan};
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent,
};
use tuirealm::{State, StateValue};

#[derive(MockComponent)]
pub struct SelectLine {
    component: List,
}

impl SelectLine {
    fn create_table() -> Table {
        let mut builder = TableBuilder::default();
        for (index, line) in LINES.iter().enumerate() {
            builder.add_col(TextSpan::from(*line));
            if index != LINES.len() - 1 {
                builder.add_row();
            }
        }
        builder.build()
    }
}

impl Default for SelectLine {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .title(fl!("select-line"), Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str("🏄‍♂️")
                .rewind(true)
                .rows(Self::create_table())
                .selected_line(0),
        }
    }
}

impl Component<Message, ApiEvent> for SelectLine {
    fn on(&mut self, event: Event<ApiEvent>) -> Option<Message> {
        let command = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Up),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Move(Direction::Down),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Message::LineSubmitted),
            _ => Cmd::None,
        };

        match self.perform(command) {
            CmdResult::Changed(State::One(StateValue::Usize(line_index))) => {
                Some(Message::LineChanged(line_index))
            }
            _ => None,
        }
    }
}
