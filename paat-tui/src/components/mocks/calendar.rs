use crate::localization::fl;

use self::attributes::CALENDAR_TITLE;
use tui_realm_stdlib::utils::get_block;
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, Borders, Color, Style},
    tui::{
        layout::{Constraint, Direction, Layout, Rect},
        widgets::{Block, Cell, Row, Table},
    },
    AttrValue, Attribute, Frame, MockComponent, Props, State,
};

pub struct Calendar {
    props: Props,
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            props: Props::default(),
        }
    }
}

mod attributes {
    pub const CALENDAR_TITLE: &str = "CALENDAR_TITLE";
}

impl Calendar {
    pub fn calendar_title<S>(mut self, calendar_title: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Custom(attributes::CALENDAR_TITLE),
            AttrValue::String(calendar_title.as_ref().to_string()),
        );
        self
    }
}

impl MockComponent for Calendar {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let calendar_title = self
                .props
                .get(Attribute::Custom(CALENDAR_TITLE))
                .unwrap()
                .unwrap_string();
            let vertical_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Min(0)].as_ref())
                .split(area);
            frame.render_widget(
                Table::new(vec![Row::new(vec![
                    Cell::from("31"),
                    Cell::from("32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("32").style(Style::default().fg(Color::Yellow)),
                ])])
                .style(Style::default().fg(Color::White))
                .header(
                    Row::new(vec![
                        Cell::from(fl!("monday-character")),
                        Cell::from(fl!("tuesday-character")),
                        Cell::from(fl!("wednesday-character")),
                        Cell::from(fl!("thursday-character")),
                        Cell::from(fl!("friday-character")),
                        Cell::from(fl!("saturday-character")),
                        Cell::from(fl!("sunday-character")),
                    ])
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(1),
                )
                .block(Block::default().title("Table"))
                .widths(&[
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .column_spacing(1)
                .block(get_block(
                    Borders::default(),
                    Some((calendar_title, Alignment::Center)),
                    false,
                    None,
                ))
                .style(Style::default()),
                vertical_chunks[0],
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}
