use crate::localization::fl;
use crate::messages::Message;
use crate::ports::ApiEvent;
use chrono::NaiveDate;
use paat_core::types::event::Event as PaatEvent;
use paat_core::types::Direction;
use tui_realm_stdlib::Table;
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{AttrValue, Attribute};
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent)]
pub struct TrackingList {
    component: Table,
}

#[derive(Clone, Default)]
pub struct TrackingListElement {
    direction: Direction,
    date: String,
    time: String,
}

impl TrackingListElement {
    pub fn new(direction: Option<Direction>, date: Option<NaiveDate>, time: &PaatEvent) -> Self {
        Self {
            direction: direction.unwrap(),
            date: date.unwrap().to_string(),
            time: time.to_string(),
        }
    }
}

impl TrackingList {
    fn render_table_header(builder: &mut TableBuilder) {
        builder
            .add_col(TextSpan::from(format!("{}", fl!("direction"))).bold())
            .add_col(TextSpan::from(format!("{}", fl!("date"),)).bold())
            .add_col(TextSpan::from(format!("{}", fl!("time"))).bold())
            .add_row()
            .add_col(TextSpan::from("  "))
            .add_row()
            .add_col(TextSpan::from("  "))
            .add_row();
    }

    pub fn build_table_rows(tracks: Vec<TrackingListElement>) -> (Attribute, AttrValue) {
        let mut builder = TableBuilder::default();
        Self::render_table_header(&mut builder);
        for track in tracks {
            builder
                .add_col(TextSpan::from(format!("{}", track.direction)))
                .add_col(TextSpan::from(format!("{}", track.date)))
                .add_col(TextSpan::from(format!("{}", track.time)))
                .add_row()
                .add_col(TextSpan::from("  "))
                .add_row();
        }
        let table_rows = builder.build();
        (Attribute::Content, AttrValue::Table(table_rows))
    }
}

impl Default for TrackingList {
    fn default() -> Self {
        let mut builder = TableBuilder::default();
        Self::render_table_header(&mut builder);
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .widths(&[40, 25, 30, 5])
                .title(fl!("track-list"), Alignment::Center)
                .table(builder.build()),
        }
    }
}

impl Component<Message, ApiEvent> for TrackingList {
    fn on(&mut self, _: Event<ApiEvent>) -> Option<Message> {
        None
    }
}
