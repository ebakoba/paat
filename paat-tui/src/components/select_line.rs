use std::time::Duration;

use tui_realm_stdlib::{List, Select};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Table, TableBuilder, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
// tui
use super::close_event_matcher;
use crate::localization::fl;
use crate::messages::Message;
use paat_core::constants::LINES;
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

#[derive(MockComponent)]
pub struct SelectLine {
    component: List,
}

impl SelectLine {
    fn create_table() -> Table {
        let mut builder = TableBuilder::default();
        for line in LINES.iter() {
            builder.add_row().add_col(TextSpan::from(*line));
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
                .step(4)
                .rows(Self::create_table())
                .selected_line(2),
        }
    }
}

impl Component<Message, NoUserEvent> for SelectLine {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        if let Some(message) = close_event_matcher(event.clone(), |_| None) {
            return Some(message);
        }

        None
    }
}
