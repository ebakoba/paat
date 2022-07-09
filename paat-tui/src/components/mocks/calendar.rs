use self::attributes::{CALENDAR_DATE, CALENDAR_TITLE};
use crate::localization::fl;
use chrono::{Datelike, NaiveDate};
use paat_core::datetime::get_naive_date_from_output_format;
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Calendar {
    props: Props,
}

mod attributes {
    pub const CALENDAR_TITLE: &str = "CALENDAR_TITLE";
    pub const CALENDAR_DATE: &str = "CALENDAR_DATE";
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

    pub fn calendar_date<S>(mut self, calendar_date: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Custom(attributes::CALENDAR_DATE),
            AttrValue::String(calendar_date.as_ref().to_string()),
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

    fn days_in_month(year: i32, month: u32) -> i64 {
        if month == 12 {
            NaiveDate::from_ymd(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd(year, month + 1, 1)
        }
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
    }

    fn cell_from_day_number<'a>(&self, year: i32, month: u32, day_number: i64) -> Cell<'a> {
        let date_string = if day_number < 10 {
            format!(" {}", day_number)
        } else {
            format!("{}", day_number)
        };
        let date = NaiveDate::from_ymd(year, month, day_number as u32);
        let current_value = get_naive_date_from_output_format(
            &self
                .props
                .get(Attribute::Custom(CALENDAR_DATE))
                .unwrap()
                .unwrap_string(),
        )
        .unwrap();
        if date == current_value {
            Cell::from(date_string).style(Style::default().fg(Color::Green))
        } else {
            Cell::from(date_string)
        }
    }

    fn create_calendar_rows<'a>(&self, year: i32, month: u32) -> Vec<Row<'a>> {
        let mut calendar_rows: Vec<Vec<Cell>> = Vec::new();

        let start_weekday = NaiveDate::from_ymd(year, month, 1)
            .weekday()
            .num_days_from_monday();
        let mut row_count = 0;
        let mut day_count = 1;
        while day_count <= Self::days_in_month(year, month) {
            if calendar_rows.get(row_count) == None {
                calendar_rows.push(Vec::new())
            }
            let current_row = calendar_rows.get_mut(row_count).unwrap();

            if row_count == 0 && start_weekday > current_row.len() as u32 {
                current_row.push(Cell::from("  "));
            } else {
                current_row.push(self.cell_from_day_number(year, month, day_count));
                day_count += 1;
            }
            if current_row.len() == 7 {
                row_count += 1;
            }
        }

        calendar_rows.into_iter().map(Row::new).collect()
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
                Table::new(self.create_calendar_rows(2022, 7))
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
                    .column_spacing(3)
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
