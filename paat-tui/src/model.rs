use crate::{
    components::{
        AppHeader, ComponentId, DepartureDate, HeaderAttributes, HiddenHandler, SelectFerry,
        SelectLine, TrackingList, TrackingListElement,
    },
    localization::fl,
    messages::Message,
    ports::{ApiClient, ApiEvent},
    style::CALENDAR_WIDTH,
};
use anyhow::Result;
use chrono::NaiveDate;
use paat_core::{
    client::Client,
    constants::TIMEOUT_BETWEEN_REQUESTS,
    datetime::get_naive_date_from_output_format,
    types::{event::EventMap, Direction as PaatDirection},
};
use std::{collections::BTreeMap, time::Duration};
use tokio::runtime::Runtime;
use tuirealm::{
    props::{PropPayload, PropValue},
    terminal::TerminalBridge,
    tui::layout::{Alignment, Constraint, Direction, Layout},
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};

#[derive(Clone, Default)]
pub struct AppState {
    departure_date: Option<NaiveDate>,
    direction: Option<PaatDirection>,
    events: EventMap,
    api_clients: Vec<ApiClient>,
    track_list: Vec<TrackingListElement>,
}

pub struct Model {
    pub app: Application<ComponentId, Message, ApiEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub state: AppState,
    pub client: Client,
    pub runtime: Runtime,
}

impl Default for Model {
    fn default() -> Self {
        let client = Client::new(Duration::from_secs(TIMEOUT_BETWEEN_REQUESTS));
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            state: AppState::default(),
            client,
            runtime,
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
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 7),
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
                app.view(&ComponentId::TrackingList, f, bottom_row[3])
            })
            .is_ok());
    }

    fn configure_listener(&mut self) -> Result<()> {
        if let Some(departure_date) = self.state.departure_date {
            if let Some(direction) = self.state.direction {
                let api_client =
                    ApiClient::try_new(&self.client, &self.runtime, departure_date, direction)?;
                self.state.api_clients.push(api_client.clone());
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

    fn reset_selection(&mut self) {
        self.state.direction = None;
        self.state.departure_date = None;
        assert!(self.app.active(&ComponentId::DepartureDate).is_ok());
        assert!(self
            .app
            .attr(
                &ComponentId::SelectLine,
                Attribute::Value,
                AttrValue::Payload(PropPayload::One(PropValue::Usize(0)))
            )
            .is_ok());
        assert!(self
            .app
            .attr(
                &ComponentId::SelectFerry,
                Attribute::Title,
                AttrValue::Title((fl!("select-date-first"), Alignment::Center))
            )
            .is_ok());
        let (attribute, value) = SelectFerry::build_table_rows(BTreeMap::new());
        assert!(self
            .app
            .attr(&ComponentId::SelectFerry, attribute, value)
            .is_ok());
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
        assert!(app
            .mount(
                ComponentId::TrackingList,
                Box::new(TrackingList::default()),
                vec![]
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
                    None
                }
                Message::LineSubmitted => {
                    let line_index = self
                        .app
                        .state(&ComponentId::SelectLine)
                        .unwrap()
                        .unwrap_one()
                        .unwrap_usize();
                    self.state.direction = PaatDirection::get_line_by_index(line_index);

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
                    self.state.events = events.clone();
                    let (attribute, value) = SelectFerry::build_table_rows(events);
                    assert!(self
                        .app
                        .attr(&ComponentId::SelectFerry, attribute, value)
                        .is_ok());
                    None
                }
                Message::FerryChanged(line_index) => {
                    assert!(self
                        .app
                        .attr(
                            &ComponentId::SelectFerry,
                            Attribute::Value,
                            AttrValue::Payload(PropPayload::One(PropValue::Usize(line_index)))
                        )
                        .is_ok());
                    None
                }
                Message::FerrySubmitted => {
                    let line_index = self
                        .app
                        .state(&ComponentId::SelectFerry)
                        .unwrap()
                        .unwrap_one()
                        .unwrap_usize();
                    let mut events = self
                        .state
                        .events
                        .values()
                        .collect::<Vec<&paat_core::types::event::Event>>();
                    events.sort_by_key(|event| event.start.clone());
                    let event = events[line_index];
                    self.state.api_clients.last().unwrap().start_monitoring(
                        &self.client,
                        &self.runtime,
                        event.uuid.clone(),
                    );
                    self.state.track_list.push(TrackingListElement::new(
                        self.state.direction,
                        self.state.departure_date,
                        event,
                    ));
                    let (attribute, value) =
                        TrackingList::build_table_rows(self.state.track_list.clone());
                    assert!(self
                        .app
                        .attr(&ComponentId::TrackingList, attribute, value)
                        .is_ok());
                    self.reset_selection();
                    None
                }
                Message::WaitResultReceived(_) => None,
                Message::TickFromListener => {
                    for element in self.state.track_list.iter_mut() {
                        element.counter += 1;
                    }
                    let (attribute, value) =
                        TrackingList::build_table_rows(self.state.track_list.clone());
                    assert!(self
                        .app
                        .attr(&ComponentId::TrackingList, attribute, value)
                        .is_ok());
                    None
                }
            }
        } else {
            None
        }
    }
}
