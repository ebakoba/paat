use crate::localization::fl;

use self::attributes::CALENDAR_TITLE;
use chrono::{Month, NaiveDate, Weekday};
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

    fn create_calendar_header<'a>() -> Row<'a> {
        let day_names = vec![
            fl!("monday-character"),
            fl!("tuesday-character"),
            fl!("wednesday-character"),
            fl!("thursday-character"),
            fl!("friday-character"),
            fl!("saturday-character"),
            fl!("sunday-character"),
        ];

        let cells: Vec<Cell> = day_names
            .iter()
            .map(|letter| Cell::from(format!(" {}", letter)))
            .collect();
        Row::new(cells).style(Style::default().fg(Color::Yellow))
    }

    fn days_in_month(year: i32, month: u32) -> Vec<i64> {
        let year = 2018;
        let last_day = if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        }
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days();
        (1..last_day).collect()
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
                Table::new(vec![])
                    .style(Style::default().fg(Color::White))
                    .header(Calendar::create_calendar_header())
                    .block(Block::default().title("Table"))
                    .widths(&[
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
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
