use crate::{
    components::{
        AppHeader, ComponentId, DepartureDate, HeaderAttributes, HiddenHandler, SelectFerry,
        SelectLine,
    },
    localization::fl,
    messages::Message,
    ports::{ApiClient, ApiEvent},
    style::CALENDAR_WIDTH,
};
use anyhow::Result;
use chrono::NaiveDate;
use paat_core::{datetime::get_naive_date_from_output_format, types::Direction as PaatDirection};
use std::time::Duration;
use tuirealm::{
    props::{PropPayload, PropValue},
    terminal::TerminalBridge,
    tui::layout::{Alignment, Constraint, Direction, Layout},
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};

#[derive(Clone, Default)]
pub struct AppState {
    intermediate_line_index: Option<usize>,
    departure_date: Option<NaiveDate>,
    direction: Option<PaatDirection>,
}

pub struct Model {
    pub app: Application<ComponentId, Message, ApiEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub state: AppState,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            state: AppState::default(),
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        let app = &mut self.app;
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(14),
                            Constraint::Length(0),
                            Constraint::Length(14),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                let vertical_fixer = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Max(35)].as_ref())
                    .split(chunks[2]);
                let bottom_row = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(CALENDAR_WIDTH),
                            Constraint::Ratio(1, 4),
                            Constraint::Ratio(1, 4),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(vertical_fixer[0]);
                app.view(&ComponentId::Header, f, chunks[0]);
                app.view(&ComponentId::HiddenHandler, f, chunks[1]);
                app.view(&ComponentId::DepartureDate, f, bottom_row[0]);
                app.view(&ComponentId::SelectLine, f, bottom_row[1]);
                app.view(&ComponentId::SelectFerry, f, bottom_row[2]);
            })
            .is_ok());
    }

    fn configure_listener(&mut self) -> Result<()> {
        if let Some(departure_date) = self.state.departure_date {
            if let Some(direction) = self.state.direction {
                let api_client = ApiClient::try_new(departure_date, direction)?;
                assert!(self
                    .app
                    .restart_listener(
                        EventListenerCfg::default()
                            .default_input_listener(Duration::from_millis(20))
                            .poll_timeout(Duration::from_millis(10))
                            .tick_interval(Duration::from_secs(1))
                            .port(Box::new(api_client), Duration::from_millis(100)),
                    )
                    .is_ok());
            }
        }
        Ok(())
    }

    fn init_app() -> Application<ComponentId, Message, ApiEvent> {
        let mut app: Application<ComponentId, Message, ApiEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
        assert!(app
            .mount(ComponentId::Header, Box::new(AppHeader::new()), vec![])
            .is_ok());
        assert!(app
            .mount(
                ComponentId::DepartureDate,
                Box::new(DepartureDate::new()),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                ComponentId::SelectLine,
                Box::new(SelectLine::default()),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                ComponentId::SelectFerry,
                Box::new(SelectFerry::default()),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(
                ComponentId::HiddenHandler,
                Box::new(HiddenHandler::default()),
                vec![Sub::new(SubEventClause::Any, SubClause::Always)]
            )
            .is_ok());
        assert!(app.active(&ComponentId::DepartureDate).is_ok());
        app
    }
}

impl Update<Message> for Model {
    fn update(&mut self, msg: Option<Message>) -> Option<Message> {
        if let Some(msg) = msg {
            self.redraw = true;
            match msg {
                Message::AppClose => {
                    self.quit = true;
                    None
                }
                Message::DepartureDateChanged(new_departure_date) => {
                    assert!(self
                        .app
                        .attr(
                            &ComponentId::DepartureDate,
                            Attribute::Value,
                            AttrValue::String(new_departure_date)
                        )
                        .is_ok());
                    None
                }
                Message::DepartureDateSubmitted(departure_date) => {
                    assert!(self.app.active(&ComponentId::SelectLine).is_ok());
                    assert!(self
                        .app
                        .attr(
                            &ComponentId::SelectFerry,
                            Attribute::Title,
                            AttrValue::Title((departure_date.clone(), Alignment::Center))
                        )
                        .is_ok());
                    self.state.departure_date =
                        get_naive_date_from_output_format(&departure_date).ok();
                    None
                }
                Message::LineChanged(line_index) => {
                    assert!(self
                        .app
                        .attr(
                            &ComponentId::SelectLine,
                            Attribute::Value,
                            AttrValue::Payload(PropPayload::One(PropValue::Usize(line_index)))
                        )
                        .is_ok());
                    self.state.intermediate_line_index = Some(line_index);
                    None
                }
                Message::LineSubmitted => {
                    self.state.direction = self
                        .state
                        .intermediate_line_index
                        .and_then(|line_index| PaatDirection::get_line_by_index(line_index));

                    match self.configure_listener() {
                        Ok(_) => {
                            assert!(self.app.active(&ComponentId::SelectFerry).is_ok());
                        }
                        Err(_) => {
                            assert!(self
                                .app
                                .attr(
                                    &ComponentId::Header,
                                    Attribute::Custom(HeaderAttributes::ERROR_TEXT),
                                    AttrValue::String(fl!("event-fetch-error"))
                                )
                                .is_ok());
                        }
                    }
                    None
                }
                Message::EventsReceived(events) => {
                    let (attribute, value) = SelectFerry::build_table_rows(events);
                    assert!(self
                        .app
                        .attr(&ComponentId::SelectFerry, attribute, value)
                        .is_ok());
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
